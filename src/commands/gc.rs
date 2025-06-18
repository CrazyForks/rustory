use anyhow::Result;
use std::env;

use crate::repository::Repository;

pub struct GcCommand;

impl GcCommand {
    pub fn execute(dry_run: bool, aggressive: bool, prune_expired: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let mut repo = Repository::new(root)?;

        repo.run_gc(dry_run, aggressive, prune_expired)?;

        Ok(())
    }
}
