use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::Repository;

pub struct InitCommand;

impl InitCommand {
    pub fn execute(path: Option<PathBuf>) -> Result<()> {
        let root = path.unwrap_or_else(|| std::env::current_dir().unwrap());

        if root.join(".rustory").exists() {
            // 重新初始化确认提示
            print!("A rustory repository already exists. Do you want to reinitialize it? (y/n): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let answer = input.trim().to_lowercase();
            if answer != "y" && answer != "yes" {
                println!("Initialization cancelled.");
                return Ok(());
            }

            // 重新初始化
            Repository::init(root)?;
            println!("Reinitialized existing rustory repository in .rustory/");
            return Ok(());
        }

        Repository::init(root)?;
        println!("Initialized empty rustory repository in .rustory/");

        Ok(())
    }
}
