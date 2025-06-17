use anyhow::Result;
use std::env;

use crate::Repository;

pub struct CommitCommand;

impl CommitCommand {
    pub fn execute(message: Option<String>, json_output: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let mut repo = Repository::new(root)?;

        let message = message.unwrap_or_else(|| "".to_string());
        let snapshot_id = repo.create_snapshot(message.clone())?;

        // 获取变更统计
        let history = repo.snapshot_manager.load_snapshot(&snapshot_id)?;

        if json_output {
            let output = serde_json::json!({
                "snapshot_id": snapshot_id,
                "timestamp": history.timestamp,
                "message": message,
                "changes": {
                    "added": history.added,
                    "modified": history.modified,
                    "deleted": history.deleted
                }
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!(
                "[snapshot {}] {}  added={} modified={} deleted={}",
                snapshot_id,
                history.timestamp.format("%Y-%m-%dT%H:%M:%S"),
                history.added,
                history.modified,
                history.deleted
            );
        }

        Ok(())
    }
}
