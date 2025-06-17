use anyhow::Result;
use std::env;

use crate::Repository;

pub struct TagCommand;

impl TagCommand {
    pub fn execute(name: String, snapshot_id: String) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let mut repo = Repository::new(root)?;

        // 验证快照是否存在
        repo.snapshot_manager.load_snapshot(&snapshot_id)?;

        // 添加标签到配置
        repo.config.set(&format!("tag.{}", name), snapshot_id.clone())?;
        repo.config.save(&repo.rustory_dir)?;

        println!("Tagged snapshot {} as \"{}\"", snapshot_id, name);

        Ok(())
    }
}
