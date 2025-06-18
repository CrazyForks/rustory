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
                // 根据配置决定显示UTC时间还是本地时间
                let time_display = if repo.config.use_local_timezone {
                    entry
                        .timestamp
                        .with_timezone(&chrono::Local)
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                } else {
                    entry.timestamp.format("%Y-%m-%dT%H:%M:%SZ").to_string()
                };

                println!(
                    "{:<8} {:<20} {:>2} {:>2} {:>2} \"{}\"",
                    entry.snapshot_id,
                    time_display,
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
