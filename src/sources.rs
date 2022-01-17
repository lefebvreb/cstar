use anyhow::{Error, Result};

use std::collections::HashSet;
use std::fs;
use std::path::{PathBuf, Path};

// A structs that holds the names of the source filed already parsed.
#[derive(Default, Debug)]
pub struct Sources {
    pub src: HashSet<PathBuf>,
}

impl Sources {
    // Adds a new source to the Sources list. If successful in reading the
    // file, will return Some static reference to the read string if
    // the file wasn't read already, or None if it was.
    pub fn add(&mut self, path: &Path) -> Result<Option<&'static str>>
    {
        self.src.insert(path.to_path_buf())
            .then(|| fs::read_to_string(&path)
                .map_err(Error::from)
                .map(|s| &*Box::leak(String::into_boxed_str(s))))
            .transpose()
    }
}