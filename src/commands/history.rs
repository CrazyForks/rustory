use anyhow::Result;
use std::env;

use crate::Repository;

pub struct HistoryCommand;

impl HistoryCommand {
    pub fn execute(json_output: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root)?;

        let history = repo.snapshot_manager.list_history()?;

        if json_output {
            println!("{}", serde_json::to_string_pretty(&history)?);
        } else {
            if history.is_empty() {
                println!("No snapshots found.");
                return Ok(());
            }

            println!(
                "{:<8} {:<20} {:>2} {:>2} {:>2} Message",
                "ID", "Time", "+", "~", "-"
            );
            println!("{}", "-".repeat(60));

            for entry in &history {
                println!(
                    "{:<8} {:<20} {:>2} {:>2} {:>2} \"{}\"",
                    entry.snapshot_id,
                    entry.timestamp.format("%Y-%m-%dT%H:%M:%S"),
                    entry.added,
                    entry.modified,
                    entry.deleted,
                    entry.message
                );
            }
        }

        Ok(())
    }
}
