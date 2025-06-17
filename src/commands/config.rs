use anyhow::Result;
use std::env;

use crate::Repository;

pub struct ConfigCommand;

impl ConfigCommand {
    pub fn execute(action: String, key: String, value: Option<String>) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let mut repo = Repository::new(root)?;

        match action.as_str() {
            "get" => {
                if let Some(val) = repo.config.get(&key) {
                    println!("{}", val);
                } else {
                    println!("Configuration key '{}' not found", key);
                }
            }
            "set" => {
                if let Some(val) = value {
                    repo.config.set(&key, val)?;
                    repo.config.save(&repo.rustory_dir)?;
                    println!("Configuration updated: {} = {}", key, repo.config.get(&key).unwrap_or_default());
                } else {
                    return Err(anyhow::anyhow!("Value required for 'set' action"));
                }
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown config action: {}", action));
            }
        }

        Ok(())
    }
}
