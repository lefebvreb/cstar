use super::*;

#[derive(Default, Debug)]
pub struct StructDef<'a> {
    pub names: Vec<&'a str>,
}