use anyhow::Result;
use std::path::PathBuf;

use crate::Repository;

pub struct InitCommand;

impl InitCommand {
    pub fn execute(path: Option<PathBuf>) -> Result<()> {
        let root = path.unwrap_or_else(|| std::env::current_dir().unwrap());
        
        if root.join(".rustory").exists() {
            println!("Reinitialized existing rustory repository in .rustory/");
            return Ok(());
        }

        Repository::init(root)?;
        println!("Initialized empty rustory repository in .rustory/");
        
        Ok(())
    }
}
