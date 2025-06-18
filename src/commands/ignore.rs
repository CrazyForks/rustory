use anyhow::Result;
use std::env;
use std::fs;
use std::process::Command;

use crate::Repository;

pub struct IgnoreCommand;

impl IgnoreCommand {
    pub fn execute(action: Option<String>) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root)?;

        let ignore_path = repo.rustory_dir.join("ignore");

        match action.as_deref() {
            Some("show") | None => {
                Self::show_ignore(&ignore_path)?;
            }
            Some("edit") => {
                Self::edit_ignore(&repo, &ignore_path)?;
            }
            Some(cmd) => {
                return Err(anyhow::anyhow!("Unknown ignore command: {}", cmd));
            }
        }

        Ok(())
    }

    fn show_ignore(ignore_path: &std::path::Path) -> Result<()> {
        if ignore_path.exists() {
            let content = fs::read_to_string(ignore_path)?;
            println!("{}", content);
        } else {
            println!("No ignore rules defined.");
        }
        Ok(())
    }

    fn edit_ignore(repo: &Repository, ignore_path: &std::path::Path) -> Result<()> {
        let editor = &repo.config.editor;

        let status = Command::new(editor).arg(ignore_path).status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Editor exited with non-zero status"));
        }

        println!("Ignore rules updated.");
        Ok(())
    }
}
