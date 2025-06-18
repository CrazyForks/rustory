use anyhow::Result;
use std::env;

use crate::{Repository, stats::StatsCollector};

pub struct StatsCommand;

impl StatsCommand {
    pub fn execute(json: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root.clone())?;
        
        let stats = StatsCollector::collect_repository_stats(&repo.rustory_dir)?;
        
        if json {
            println!("{}", serde_json::to_string_pretty(&stats)?);
        } else {
            StatsCollector::print_stats(&stats);
        }
        
        Ok(())
    }
}
