use anyhow::{Result, anyhow};
use std::env;
use std::io::{self, Write};

use crate::{Repository, stats::StatsCollector};

pub struct UtilsCommand;

impl UtilsCommand {
    /// 垃圾回收功能
    pub fn gc(dry_run: bool, aggressive: bool, prune_expired: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let mut repo = Repository::new(root)?;

        repo.run_gc(dry_run, aggressive, prune_expired)?;

        Ok(())
    }

    /// 显示仓库统计信息
    pub fn stats(json: bool) -> Result<()> {
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

    /// 验证仓库完整性
    pub fn verify(fix: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root.clone())?;

        println!("🔍 Verifying repository integrity...");

        // 检查对象存储一致性
        let corrupted_objects = Self::verify_object_consistency(&repo.rustory_dir.join("objects"))?;

        if corrupted_objects.is_empty() {
            println!("All objects are consistent");
        } else {
            println!("Found {} corrupted objects:", corrupted_objects.len());
            for obj in &corrupted_objects {
                println!("  - {}", obj);
            }

            if fix {
                println!("🔧 Attempting to repair...");
                // 这里可以实现修复逻辑
                println!("Repair functionality not yet implemented");
            }
        }

        // 验证所有快照
        let snapshots_dir = repo.rustory_dir.join("snapshots");
        let mut verified_snapshots = 0;
        let mut failed_snapshots = 0;

        if snapshots_dir.exists() {
            for entry in std::fs::read_dir(&snapshots_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    match Self::verify_snapshot_integrity(&path) {
                        Ok(_) => verified_snapshots += 1,
                        Err(_) => {
                            failed_snapshots += 1;
                            println!("Snapshot verification failed: {}", path.display());
                        }
                    }
                }
            }
        }

        println!("\nVerification Summary:");
        println!("  Verified snapshots: {}", verified_snapshots);
        println!("  Failed snapshots: {}", failed_snapshots);
        println!("  Corrupted objects: {}", corrupted_objects.len());

        if failed_snapshots == 0 && corrupted_objects.is_empty() {
            println!("Repository is healthy!");
        } else {
            println!("Repository has integrity issues that need attention");
        }

        Ok(())
    }

    /// 交互式选择要提交的文件
    pub fn select_files_to_commit() -> Result<Vec<String>> {
        println!("Interactive commit mode:");
        println!("Select files to include in this commit:");

        // 这里可以实现交互式文件选择逻辑
        // 暂时返回空向量作为示例
        Ok(vec![])
    }

    /// 交互式查看更改
    pub fn review_changes() -> Result<bool> {
        print!("Review changes before commit? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase() == "y")
    }

    /// 简化的快照验证 - 仅检查文件是否为有效JSON
    fn verify_snapshot_integrity(snapshot_path: &std::path::Path) -> Result<bool> {
        let content = std::fs::read_to_string(snapshot_path)?;

        // 简单验证：检查是否能成功解析为快照元数据
        match serde_json::from_str::<crate::SnapshotMetadata>(&content) {
            Ok(snapshot) => {
                println!("Snapshot {} integrity: OK", snapshot.id);
                Ok(true)
            }
            Err(e) => {
                println!("Snapshot integrity check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// 简化的对象存储检查 - 仅检查对象文件是否存在且可读
    fn verify_object_consistency(objects_dir: &std::path::Path) -> Result<Vec<String>> {
        let mut corrupted_objects = Vec::new();

        if !objects_dir.exists() {
            return Ok(corrupted_objects);
        }

        // 简单检查：遍历对象目录，验证文件是否可读
        for entry in walkdir::WalkDir::new(objects_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.path();

                // 尝试读取文件
                match std::fs::read(path) {
                    Ok(_) => {
                        // 文件可读，无需进一步验证
                        continue;
                    }
                    Err(_) => {
                        // 文件不可读，标记为损坏
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            corrupted_objects.push(name.to_string());
                        }
                    }
                }
            }
        }

        Ok(corrupted_objects)
    }

    /// 删除特定的快照
    pub fn remove_snapshots(target: String, dry_run: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root)?;

        if dry_run {
            println!("Running in dry-run mode (no changes will be made)");
        }

        // 解析目标参数 (可能是 number, ID, number-number, ID-ID 格式)
        if target.contains('-') {
            // 范围删除
            Self::remove_snapshot_range(&repo, target, dry_run)?;
        } else {
            // 单个删除
            Self::remove_single_snapshot(&repo, target, dry_run)?;
        }

        Ok(())
    }

    /// 删除单个快照
    fn remove_single_snapshot(repo: &Repository, target: String, dry_run: bool) -> Result<()> {
        let snapshot_id = if target.chars().all(|c| c.is_ascii_digit()) {
            // 如果是纯数字，按序号查找
            let number: usize = target.parse()?;
            repo.snapshot_manager.find_snapshot_by_number(number)?
        } else {
            // 否则当作ID处理
            target
        };

        if dry_run {
            println!("Would remove snapshot: {}", snapshot_id);
        } else {
            repo.snapshot_manager.delete_snapshot(&snapshot_id)?;
            println!("Removed snapshot: {}", snapshot_id);
        }

        Ok(())
    }

    /// 删除范围内的快照
    fn remove_snapshot_range(repo: &Repository, range: String, dry_run: bool) -> Result<()> {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid range format. Use number-number or ID-ID"));
        }

        let start_str = parts[0];
        let end_str = parts[1];

        // 获取所有历史记录
        let history = repo.snapshot_manager.list_history()?;

        let snapshots_to_remove = if start_str.chars().all(|c| c.is_ascii_digit())
            && end_str.chars().all(|c| c.is_ascii_digit())
        {
            // 数字范围
            let start_num: usize = start_str.parse()?;
            let end_num: usize = end_str.parse()?;

            if start_num > end_num {
                return Err(anyhow!("Invalid range: start number must be <= end number"));
            }

            history
                .iter()
                .filter(|entry| entry.number >= start_num && entry.number <= end_num)
                .map(|entry| entry.snapshot_id.clone())
                .collect::<Vec<_>>()
        } else {
            // ID范围 - 在历史记录中找到这两个ID之间的所有快照
            let start_idx = history.iter().position(|e| e.snapshot_id == start_str);
            let end_idx = history.iter().position(|e| e.snapshot_id == end_str);

            match (start_idx, end_idx) {
                (Some(start), Some(end)) => {
                    let (from, to) = if start <= end {
                        (start, end)
                    } else {
                        (end, start)
                    };
                    history[from..=to]
                        .iter()
                        .map(|entry| entry.snapshot_id.clone())
                        .collect()
                }
                _ => {
                    return Err(anyhow!("One or both snapshot IDs not found"));
                }
            }
        };

        if snapshots_to_remove.is_empty() {
            println!("No snapshots found in the specified range");
            return Ok(());
        }

        println!("Found {} snapshots to remove", snapshots_to_remove.len());

        for snapshot_id in &snapshots_to_remove {
            if dry_run {
                println!("Would remove snapshot: {}", snapshot_id);
            } else {
                repo.snapshot_manager.delete_snapshot(snapshot_id)?;
                println!("Removed snapshot: {}", snapshot_id);
            }
        }

        Ok(())
    }
}
