use anyhow::Result;
use std::env;
use std::path::Path;
use walkdir::WalkDir;

use crate::{Repository, utils};

pub struct RollbackCommand;

impl RollbackCommand {
    pub fn execute(snapshot_id: String, restore: bool, keep_index: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root.clone())?;

        let snapshot_id = if snapshot_id.parse::<usize>().is_ok() {
            // 如果 snapshot_id 是数字，尝试通过快照编号解析
            repo.resolve_snapshot_number(&repo, snapshot_id.parse::<usize>()?)?
        } else {
            // 否则直接使用 snapshot_id
            snapshot_id
        };

        if restore {
            // 直接恢复到工作区
            Self::restore_to_working_dir(&repo, &root, &snapshot_id, keep_index)?;
        } else {
            // 导出到备份目录
            Self::export_to_backup(&repo, &root, &snapshot_id)?;
        }

        Ok(())
    }

    fn restore_to_working_dir(
        repo: &Repository,
        root: &Path,
        snapshot_id: &str,
        keep_index: bool,
    ) -> Result<()> {
        // 创建备份
        let backup_dir = root.join(utils::create_backup_name());
        std::fs::create_dir_all(&backup_dir)?;

        // 备份当前工作区
        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if path.is_file() {
                let relative_path = path.strip_prefix(root)?;

                // 跳过 .rustory 目录和 rustory-rollback 目录
                if relative_path.starts_with(".rustory")
                    || relative_path.starts_with("rustory-rollback")
                {
                    continue;
                }

                let backup_path = backup_dir.join(relative_path);
                if let Some(parent) = backup_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::copy(path, backup_path)?;
            }
        }

        // 清空工作区（除了 .rustory 和 rustory-rollback）
        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if path.is_file() {
                let relative_path = path.strip_prefix(root)?;

                // 跳过 .rustory 目录和 rustory-rollback 目录
                if relative_path.starts_with(".rustory")
                    || relative_path.starts_with("rustory-rollback")
                {
                    continue;
                }

                std::fs::remove_file(path)?;
            }
        }

        // 恢复快照内容
        repo.snapshot_manager
            .restore_snapshot(snapshot_id, root, &repo.object_store)?;

        // 更新索引（如果不保持索引）
        if !keep_index {
            let snapshot = repo.snapshot_manager.load_snapshot(snapshot_id)?;
            let index = crate::Index {
                files: snapshot.files,
            };
            repo.index_manager.save(&index)?;
        }

        println!("Restored snapshot {} to working directory", snapshot_id);
        println!("Original files backed up to {}", backup_dir.display());

        Ok(())
    }

    fn export_to_backup(repo: &Repository, root: &Path, snapshot_id: &str) -> Result<()> {
        let backup_dir = root.join(utils::create_backup_name());

        repo.snapshot_manager
            .restore_snapshot(snapshot_id, &backup_dir, &repo.object_store)?;

        println!(
            "Exported snapshot {} to {}",
            snapshot_id,
            backup_dir.display()
        );

        Ok(())
    }
}

impl Repository {
    pub fn resolve_snapshot_number(&self, repo: &Repository, number: usize) -> Result<String> {
        let history = repo.snapshot_manager.list_history()?;
        history
            .iter()
            .find(|entry| entry.number == number)
            .map(|entry| entry.snapshot_id.clone())
            .ok_or_else(|| anyhow::anyhow!("Snapshot number {} not found", number))
    }
}
