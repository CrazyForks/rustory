use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::index::IndexManager;
use crate::objects::ObjectStore;
use crate::snapshot::SnapshotManager;

pub struct Repository {
    pub root: PathBuf,
    pub rustory_dir: PathBuf,
    pub config: Config,
    pub object_store: ObjectStore,
    pub index_manager: IndexManager,
    pub snapshot_manager: SnapshotManager,
}

impl Repository {
    pub fn new(root: PathBuf) -> Result<Self> {
        let rustory_dir = root.join(".rustory");
        
        if !rustory_dir.exists() {
            return Err(anyhow!("fatal: not a rustory repository (or any parent up to root)"));
        }

        let config = Config::load(&rustory_dir)?;
        let object_store = ObjectStore::new(rustory_dir.join("objects"));
        let index_manager = IndexManager::new(rustory_dir.join("index.json"));
        let snapshot_manager = SnapshotManager::new(
            rustory_dir.join("snapshots"),
            rustory_dir.join("history.log"),
        );

        Ok(Self {
            root,
            rustory_dir,
            config,
            object_store,
            index_manager,
            snapshot_manager,
        })
    }

    pub fn init(root: PathBuf) -> Result<Self> {
        let rustory_dir = root.join(".rustory");

        // 创建目录结构
        fs::create_dir_all(&rustory_dir)?;
        fs::create_dir_all(rustory_dir.join("objects"))?;
        fs::create_dir_all(rustory_dir.join("snapshots"))?;

        // 创建默认忽略文件（先创建这个文件，这样在扫描时就能被使用）
        let ignore_content = r#"# rustory ignore rules (gitignore style)
*.tmp
*.log
.DS_Store
Thumbs.db
*.swp
*.swo
*~

# Build artifacts
target/
build/
dist/
out/

# IDE files
.vscode/
.idea/
*.iml

# rustory itself
.rustory/

# rustory rollback directory
rustory-rollback/
"#;
        fs::write(rustory_dir.join("ignore"), ignore_content)?;

        // 创建默认配置
        let config = Config::default();
        config.save(&rustory_dir)?;

        let object_store = ObjectStore::new(rustory_dir.join("objects"));
        let index_manager = IndexManager::new(rustory_dir.join("index.json"));
        let snapshot_manager = SnapshotManager::new(
            rustory_dir.join("snapshots"),
            rustory_dir.join("history.log"),
        );

        let mut repo = Self {
            root,
            rustory_dir,
            config,
            object_store,
            index_manager,
            snapshot_manager,
        };

        // 创建初始快照
        let message = "Initial commit".to_string();
        repo.create_snapshot(message)?;

        Ok(repo)
    }

    pub fn find_root(start: &Path) -> Result<PathBuf> {
        let mut current = start.to_path_buf();
        loop {
            if current.join(".rustory").exists() {
                return Ok(current);
            }
            if !current.pop() {
                return Err(anyhow!("fatal: not a rustory repository (or any parent up to root)"));
            }
        }
    }

    pub fn create_snapshot(&mut self, message: String) -> Result<String> {
        let snapshot_id = self.snapshot_manager.create_snapshot(
            &self.root,
            &self.config,
            &mut self.object_store,
            &mut self.index_manager,
            message,
        )?;

        // 如果启用了自动 GC，在创建快照后运行
        if self.config.gc_auto_enabled {
            if let Err(e) = self.auto_gc() {
                eprintln!("Warning: Auto GC failed: {}", e);
            }
        }

        Ok(snapshot_id)
    }

    /// 执行自动垃圾回收
    pub fn auto_gc(&mut self) -> Result<()> {
        // 检查是否需要运行 GC
        if self.should_run_gc()? {
            self.run_gc(false, false, false)?;
        }
        Ok(())
    }

    /// 运行垃圾回收
    pub fn run_gc(&mut self, dry_run: bool, _aggressive: bool, prune_expired: bool) -> Result<()> {
        if dry_run {
            println!("Running in dry-run mode (no changes will be made)");
        }

        // 收集所有被引用的对象哈希
        let referenced_objects = self.collect_referenced_objects()?;
        println!("Found {} objects referenced by snapshots", referenced_objects.len());

        // 查找所有存储的对象
        let stored_objects = self.object_store.list_all_objects()?;
        println!("Found {} objects in storage", stored_objects.len());

        // 找出未被引用的对象
        let mut unreferenced_objects = Vec::new();
        for object_hash in &stored_objects {
            if !referenced_objects.contains(object_hash) {
                unreferenced_objects.push(object_hash.clone());
            }
        }

        println!("Found {} unreferenced objects", unreferenced_objects.len());

        // 删除未被引用的对象
        let mut removed_count = 0;
        let mut freed_bytes = 0u64;

        for object_hash in &unreferenced_objects {
            if !dry_run {
                if let Ok(actual_size) = self.object_store.remove_object(object_hash) {
                    removed_count += 1;
                    freed_bytes += actual_size;
                }
            } else {
                if let Ok(size) = self.object_store.get_object_size(object_hash) {
                    freed_bytes += size;
                }
                println!("Would remove object: {}", object_hash);
            }
        }

        // 如果启用了清理过期快照
        if prune_expired {
            self.prune_expired_snapshots(dry_run)?;
        }

        if dry_run {
            println!("Dry run completed. Would have:");
            println!("  - Removed {} unreferenced objects", unreferenced_objects.len());
            println!("  - Freed {} bytes ({:.2} MB)", freed_bytes, freed_bytes as f64 / 1024.0 / 1024.0);
        } else {
            println!("Garbage collection completed:");
            println!("  - Removed {} unreferenced objects", removed_count);
            println!("  - Freed {} bytes ({:.2} MB)", freed_bytes, freed_bytes as f64 / 1024.0 / 1024.0);
        }

        Ok(())
    }

    /// 检查是否应该运行 GC
    fn should_run_gc(&self) -> Result<bool> {
        // 简单的策略：每 10 次提交运行一次 GC
        let history = self.snapshot_manager.list_history()?;
        Ok(history.len() % 10 == 0)
    }

    /// 收集所有被快照引用的对象哈希
    fn collect_referenced_objects(&self) -> Result<std::collections::HashSet<String>> {
        use std::collections::HashSet;
        use std::fs;
        
        let mut referenced = HashSet::new();
        
        // 读取所有快照文件
        let snapshots_dir = self.rustory_dir.join("snapshots");
        if snapshots_dir.exists() {
            for entry in fs::read_dir(&snapshots_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(snapshot) = serde_json::from_str::<crate::SnapshotMetadata>(&content) {
                            // 收集快照中所有文件的哈希
                            for (_, file_entry) in &snapshot.files {
                                referenced.insert(file_entry.hash.clone());
                            }
                        }
                    }
                }
            }
        }
        
        Ok(referenced)
    }

    /// 清理过期的快照
    fn prune_expired_snapshots(&self, dry_run: bool) -> Result<()> {
        use chrono::{Duration, Utc};
        
        // 从配置中获取保留策略
        let keep_days = self.config.gc_keep_days.unwrap_or(30);
        let keep_count = self.config.gc_keep_snapshots.unwrap_or(50);
        
        let cutoff_date = Utc::now() - Duration::days(keep_days as i64);
        
        // 获取所有快照的历史
        let history = self.snapshot_manager.list_history()?;
        
        // 按时间排序（最新的在前）
        let mut snapshots_to_remove = Vec::new();
        
        // 保留最新的 keep_count 个快照
        for (i, entry) in history.iter().enumerate() {
            if i >= keep_count || entry.timestamp < cutoff_date {
                snapshots_to_remove.push(entry.snapshot_id.clone());
            }
        }
        
        println!("Found {} snapshots to prune", snapshots_to_remove.len());
        
        for snapshot_id in &snapshots_to_remove {
            let snapshot_path = self.rustory_dir.join("snapshots").join(format!("{}.json", snapshot_id));
            
            if !dry_run {
                if snapshot_path.exists() {
                    fs::remove_file(&snapshot_path)?;
                    println!("Removed snapshot: {}", snapshot_id);
                }
            } else {
                println!("Would remove snapshot: {}", snapshot_id);
            }
        }
        
        Ok(())
    }
}
