use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::index::IndexManager;
use crate::objects::ObjectStore;
use crate::{HistoryEntry, SnapshotMetadata};

pub struct SnapshotManager {
    snapshots_dir: PathBuf,
    history_path: PathBuf,
}

impl SnapshotManager {
    pub fn new(snapshots_dir: PathBuf, history_path: PathBuf) -> Self {
        Self {
            snapshots_dir,
            history_path,
        }
    }

    pub fn create_snapshot(
        &mut self,
        root: &Path,
        config: &Config,
        object_store: &mut ObjectStore,
        index_manager: &mut IndexManager,
        message: String,
    ) -> Result<String> {
        // 创建一个虚拟的忽略匹配器（现在在 scan_directory 内部处理）
        let dummy_matcher = ignore::gitignore::GitignoreBuilder::new(root).build()?;

        // 扫描当前目录状态
        let new_index = index_manager.scan_directory(root, &dummy_matcher)?;
        let old_index = index_manager.load().unwrap_or_else(|_| crate::Index::new());

        // 计算变更
        let mut added = 0;
        let mut modified = 0;
        let mut deleted = 0;

        // 存储新文件到对象存储
        for (path, entry) in &new_index.files {
            let full_path = root.join(path);
            if full_path.exists() {
                // 检查文件大小
                if entry.size > config.max_file_size_mb * 1024 * 1024 {
                    eprintln!(
                        "Warning: Skipping large file: {} ({}MB)",
                        path.display(),
                        entry.size / 1024 / 1024
                    );
                    continue;
                }

                object_store.store_file(&full_path)?;

                match old_index.files.get(path) {
                    None => added += 1,
                    Some(old_entry) => {
                        if old_entry.hash != entry.hash {
                            modified += 1;
                        }
                    }
                }
            }
        }

        // 计算删除的文件
        for path in old_index.files.keys() {
            if !new_index.files.contains_key(path) {
                deleted += 1;
            }
        }

        // 生成快照ID（基于时间戳、随机数和内容的哈希）
        let timestamp = chrono::Utc::now();
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        timestamp
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .hash(&mut hasher);
        message.hash(&mut hasher);
        std::process::id().hash(&mut hasher); // 进程ID增加唯一性

        // 为了进一步避免冲突，加入一些随机性
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();
        nanos.hash(&mut hasher);

        let hash = hasher.finish();
        let snapshot_id = format!("{:x}", hash).chars().take(8).collect::<String>();

        // 创建快照元数据
        let snapshot = SnapshotMetadata {
            id: snapshot_id.clone(),
            timestamp,
            message: message.clone(),
            added,
            modified,
            deleted,
            files: new_index.files.clone(),
        };

        // 保存快照元数据
        let snapshot_path = self.snapshots_dir.join(format!("{}.json", snapshot_id));
        std::fs::create_dir_all(&self.snapshots_dir)?;
        let content = serde_json::to_string_pretty(&snapshot)?;
        std::fs::write(snapshot_path, content)?;

        // 更新索引
        index_manager.save(&new_index)?;

        // 写入历史日志
        let history_entry = HistoryEntry {
            snapshot_id: snapshot_id.clone(),
            timestamp,
            added,
            modified,
            deleted,
            message,
        };
        self.append_history(&history_entry)?;

        Ok(snapshot_id)
    }

    pub fn load_snapshot(&self, snapshot_id: &str) -> Result<SnapshotMetadata> {
        let snapshot_path = self.snapshots_dir.join(format!("{}.json", snapshot_id));
        if !snapshot_path.exists() {
            return Err(anyhow::anyhow!(
                "error: snapshot '{}' not found",
                snapshot_id
            ));
        }

        let content = std::fs::read_to_string(snapshot_path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn list_history(&self) -> Result<Vec<HistoryEntry>> {
        let mut entries = Vec::new();

        if self.history_path.exists() {
            let file = File::open(&self.history_path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                if let Ok(entry) = self.parse_history_line(&line) {
                    entries.push(entry);
                }
            }
        }

        entries.reverse(); // 最新的在前面
        Ok(entries)
    }

    fn append_history(&self, entry: &HistoryEntry) -> Result<()> {
        let line = format!(
            "{} {} {}/{}/{} msg=\"{}\"\n",
            entry.snapshot_id,
            entry.timestamp.format("%Y-%m-%dT%H:%M:%S%.3fZ"),
            entry.added,
            entry.modified,
            entry.deleted,
            entry.message
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.history_path)?;

        file.write_all(line.as_bytes())?;
        Ok(())
    }

    fn parse_history_line(&self, line: &str) -> Result<HistoryEntry> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(anyhow::anyhow!("Invalid history line format"));
        }

        let snapshot_id = parts[0].to_string();
        let timestamp = chrono::DateTime::parse_from_rfc3339(parts[1])?.with_timezone(&chrono::Utc);

        let changes: Vec<&str> = parts[2].split('/').collect();
        if changes.len() != 3 {
            return Err(anyhow::anyhow!("Invalid changes format"));
        }

        let added = changes[0].parse()?;
        let modified = changes[1].parse()?;
        let deleted = changes[2].parse()?;

        // 解析消息（在 msg="..." 之间）
        let msg_start = line.find("msg=\"").map(|i| i + 5);
        let msg_end = line.rfind('"');
        let message = match (msg_start, msg_end) {
            (Some(start), Some(end)) if start < end => line[start..end].to_string(),
            _ => String::new(),
        };

        Ok(HistoryEntry {
            snapshot_id,
            timestamp,
            added,
            modified,
            deleted,
            message,
        })
    }

    pub fn restore_snapshot(
        &self,
        snapshot_id: &str,
        target_dir: &Path,
        object_store: &ObjectStore,
    ) -> Result<()> {
        let snapshot = self.load_snapshot(snapshot_id)?;

        std::fs::create_dir_all(target_dir)?;

        for (path, entry) in &snapshot.files {
            let target_path = target_dir.join(path);
            object_store.restore_file(&entry.hash, &target_path)?;
        }

        Ok(())
    }
}
