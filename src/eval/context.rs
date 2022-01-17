use std::cell::RefCell;

use super::*;

// Holds all variables ctxs.
#[derive(Debug)]
pub struct Context {
    defs: Map<Def>,
    vars: RefCell<Vec<Map<Var>>>,
}

impl Context {
    // Defines a new object in the context, retunrns an error if an
    // object with the same name already exists.
    pub fn set_def(&mut self, name: &'static str, def: Def) -> Result<()> {
        self.defs.insert(name, def)
            .is_none().then(|| ())
            .ok_or_else(|| anyhow!("object with name '{}' already exists", name))
    }

    // Returns the definition corresponding to the given name, or an
    // error if no such definition exists.
    pub fn get_def(&self, name: &str) -> Result<Def> {
        self.defs.get(name)
            .ok_or_else(|| anyhow!("Definition {} does not exist", name))
            .map(Def::clone)
    }

    // Nests another new empty ctx.
    pub fn next(&self) {
        self.vars.borrow_mut().push(Map::new());
    }

    // Destroys the last created ctx, freeing all of it's variables.
    pub fn back(&self) {
        self.vars.borrow_mut().pop();
    }

    // Adds a new variable to the topmost ctx.
    pub fn new_var(&self, name: &'static str, val: Var) {
        self.vars.borrow_mut().last_mut().unwrap().insert(name, val);
    }

    // Gets a copy of the requested variable.
    pub fn get_var(&self, name: &'static str) -> Result<Var> {
        self.vars.borrow().iter().rev()
            .find_map(|ctx| ctx.get(name))
            .map(Var::clone)
            .ok_or_else(|| anyhow!("Variable {} does not exist in current ctx.", name))
    }

    // Sets the value of the requested variable.
    pub fn set_var(&self, name: &'static str, val: Var) -> Result<()> {
        let mut borrow = self.vars.borrow_mut();
        let var = borrow.iter_mut().rev()
            .find_map(|ctx| ctx.get_mut(name))
            .ok_or_else(|| anyhow!("Variable {} does not exist in current ctx.", name))?;

        if matches!(var, Var::Struct(_) | Var::List(_)) {
            return Err(anyhow!("Cannot reassign to a struct or list variable in a ctx."));
        }

        *var = val;
        Ok(())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            defs: Map::new(),
            vars: RefCell::new(vec![Map::new()]),
        }
    }
}

// A definition of a struct-like or function-like object.
#[derive(Clone, Debug)]
pub enum Def {
    Function(&'static ast::Function),
    System(&'static ast::System),
    Component(&'static ast::StructDef),
    Resource(&'static ast::StructDef),
    Struct(&'static ast::StructDef),
}

// The result of the evaluation of a statement.
#[derive(Debug)]
pub enum Flow {
    Ok,
    Break,
    Continue,
    Return(Var),
}