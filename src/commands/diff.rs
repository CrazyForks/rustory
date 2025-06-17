use anyhow::Result;
use std::env;

use crate::Repository;

pub struct DiffCommand;

impl DiffCommand {
    pub fn execute(id1: Option<String>, id2: Option<String>) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root.clone())?;

        match (id1, id2) {
            (None, None) => {
                // 与当前工作区比较
                Self::diff_with_working_dir(&repo, &root)?;
            }
            (Some(id), None) => {
                // 指定快照与当前工作区比较
                Self::diff_snapshot_with_working_dir(&repo, &root, &id)?;
            }
            (Some(id1), Some(id2)) => {
                // 两个快照之间比较
                Self::diff_snapshots(&repo, &id1, &id2)?;
            }
            (None, Some(_)) => {
                return Err(anyhow::anyhow!("Invalid arguments: second ID provided without first ID"));
            }
        }

        Ok(())
    }

    fn diff_with_working_dir(repo: &Repository, root: &std::path::Path) -> Result<()> {
        // 创建一个虚拟的忽略匹配器（现在在内部处理）
        let dummy_matcher = ignore::gitignore::GitignoreBuilder::new(root).build()?;

        let (added, modified, deleted) = repo.index_manager
            .compare_with_current(root, &dummy_matcher)?;

        println!("diff --rustory");
        
        for path in &added {
            println!("+ {}", path.display());
        }
        
        for path in &modified {
            println!("~ {}", path.display());
        }
        
        for path in &deleted {
            println!("- {}", path.display());
        }

        Ok(())
    }

    fn diff_snapshot_with_working_dir(repo: &Repository, root: &std::path::Path, snapshot_id: &str) -> Result<()> {
        let snapshot = repo.snapshot_manager.load_snapshot(snapshot_id)?;
        let current_index = {
            let dummy_matcher = ignore::gitignore::GitignoreBuilder::new(root).build()?;
            repo.index_manager.scan_directory(root, &dummy_matcher)?
        };

        println!("diff --rustory {} current", snapshot_id);

        // 简单的文件级差异
        for (path, _) in &current_index.files {
            if !snapshot.files.contains_key(path) {
                println!("+ {}", path.display());
            }
        }

        for (path, entry) in &snapshot.files {
            match current_index.files.get(path) {
                None => println!("- {}", path.display()),
                Some(current_entry) => {
                    if entry.hash != current_entry.hash {
                        println!("~ {}", path.display());
                    }
                }
            }
        }

        Ok(())
    }

    fn diff_snapshots(repo: &Repository, id1: &str, id2: &str) -> Result<()> {
        let snapshot1 = repo.snapshot_manager.load_snapshot(id1)?;
        let snapshot2 = repo.snapshot_manager.load_snapshot(id2)?;

        println!("diff --rustory {} {}", id1, id2);

        // 简单的文件级差异
        for (path, _) in &snapshot2.files {
            if !snapshot1.files.contains_key(path) {
                println!("+ {}", path.display());
            }
        }

        for (path, entry1) in &snapshot1.files {
            match snapshot2.files.get(path) {
                None => println!("- {}", path.display()),
                Some(entry2) => {
                    if entry1.hash != entry2.hash {
                        println!("~ {}", path.display());
                    }
                }
            }
        }

        Ok(())
    }
}
