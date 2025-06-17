use anyhow::Result;
use std::env;

use crate::Repository;

pub struct StatusCommand;

impl StatusCommand {
    pub fn execute() -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root.clone())?;

        // 创建一个虚拟的忽略匹配器（现在在内部处理）
        let dummy_matcher = ignore::gitignore::GitignoreBuilder::new(&root).build()?;

        let (added, modified, deleted) = repo.index_manager
            .compare_with_current(&root, &dummy_matcher)?;

        if added.is_empty() && modified.is_empty() && deleted.is_empty() {
            println!("Working directory clean");
            return Ok(());
        }

        if !added.is_empty() {
            println!("Added:");
            for path in &added {
                println!("    {}", path.display());
            }
        }

        if !modified.is_empty() {
            println!("Modified:");
            for path in &modified {
                println!("    {}", path.display());
            }
        }

        if !deleted.is_empty() {
            println!("Deleted:");
            for path in &deleted {
                println!("    {}", path.display());
            }
        }

        Ok(())
    }
}
