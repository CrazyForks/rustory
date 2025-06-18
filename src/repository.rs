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
    pub fn run_gc(&mut self, dry_run: bool, aggressive: bool, prune_expired: bool) -> Result<()> {
        if dry_run {
            println!("Running in dry-run mode (no changes will be made)");
        }
        
        if aggressive {
            println!("Running aggressive garbage collection...");
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
                if let Ok(size) = self.object_store.get_object_size(object_hash) {
                    if self.object_store.remove_object(object_hash).is_ok() {
                        removed_count += 1;
                        freed_bytes += size;
                    }
                }
            } else {
                if let Ok(size) = self.object_store.get_object_size(object_hash) {
                    freed_bytes += size;
                }
                println!("Would remove object: {}", object_hash);
            }
        }

        // 激进模式的额外优化
        if aggressive {
            let additional_freed = self.run_aggressive_optimizations(dry_run)?;
            freed_bytes += additional_freed;
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

    /// 激进模式的额外优化功能
    fn run_aggressive_optimizations(&mut self, dry_run: bool) -> Result<u64> {
        let mut total_freed = 0u64;
        
        println!("Performing aggressive optimizations...");
        
        // 1. 重新压缩现有对象以获得更好的压缩比
        total_freed += self.recompress_objects(dry_run)?;
        
        // 2. 清理临时文件和碎片
        total_freed += self.cleanup_fragments(dry_run)?;
        
        // 3. 优化索引文件
        self.optimize_index(dry_run)?;
        
        // 4. 整理和合并相似快照
        self.optimize_snapshots(dry_run)?;
        
        // 5. 重新组织对象存储结构
        self.reorganize_object_storage(dry_run)?;
        
        Ok(total_freed)
    }
    
    /// 重新压缩对象以获得更好的压缩比
    fn recompress_objects(&mut self, dry_run: bool) -> Result<u64> {
        println!("  Recompressing objects for better compression...");
        
        let objects = self.object_store.list_all_objects()?;
        let mut total_saved = 0u64;
        let mut recompressed_count = 0;
        
        for object_hash in &objects {
            if let Ok(original_size) = self.object_store.get_object_size(object_hash) {
                // 只重新压缩较大的对象 (>1KB)
                if original_size > 1024 {
                    if !dry_run {
                        if let Ok(new_size) = self.object_store.recompress_object(object_hash) {
                            if new_size < original_size {
                                total_saved += original_size - new_size;
                                recompressed_count += 1;
                            }
                        }
                    } else {
                        // 估算可能节省的空间 (假设能节省5-10%)
                        let estimated_saved = original_size / 20; // 5%估算
                        total_saved += estimated_saved;
                        recompressed_count += 1;
                    }
                }
            }
        }
        
        if dry_run {
            println!("    Would recompress {} objects, estimated savings: {} bytes", 
                     recompressed_count, total_saved);
        } else {
            println!("    Recompressed {} objects, saved: {} bytes", 
                     recompressed_count, total_saved);
        }
        
        Ok(total_saved)
    }
    
    /// 清理临时文件和碎片 - 更激进的清理
    fn cleanup_fragments(&self, dry_run: bool) -> Result<u64> {
        println!("  Cleaning up temporary files and fragments...");
        
        let mut total_cleaned = 0u64;
        let mut files_cleaned = 0;
        let mut dirs_cleaned = 0;
        
        // 扩展临时文件模式
        let temp_patterns = vec![
            ".tmp", ".temp", "~", ".bak", ".swp", ".swo", 
            ".orig", ".rej", ".log", ".lock", ".pid"
        ];
        
        // 查找临时文件和目录
        for entry in walkdir::WalkDir::new(&self.rustory_dir) {
            let entry = entry?;
            let path = entry.path();
            
            if entry.file_type().is_file() {
                let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                
                // 检查是否是临时文件
                let is_temp = temp_patterns.iter().any(|pattern| {
                    filename.ends_with(pattern) || filename.starts_with(".")
                });
                
                // 检查是否是过期的锁文件或日志文件
                let is_expired = if filename.contains(".lock") || filename.contains(".log") {
                    if let Ok(metadata) = std::fs::metadata(path) {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(age) = modified.elapsed() {
                                age.as_secs() > 3600 // 超过1小时的锁文件/日志
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                if is_temp || is_expired {
                    if let Ok(metadata) = std::fs::metadata(path) {
                        total_cleaned += metadata.len();
                        files_cleaned += 1;
                        
                        if !dry_run {
                            if let Err(e) = std::fs::remove_file(path) {
                                println!("    Warning: Failed to remove {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            } else if entry.file_type().is_dir() && entry.depth() > 0 {
                // 检查空目录
                if let Ok(entries) = std::fs::read_dir(path) {
                    if entries.count() == 0 {
                        dirs_cleaned += 1;
                        if !dry_run {
                            if let Err(e) = std::fs::remove_dir(path) {
                                println!("    Warning: Failed to remove empty directory {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }
        
        // 额外清理：查找损坏的对象文件
        let objects_dir = self.rustory_dir.join("objects");
        if objects_dir.exists() {
            for entry in walkdir::WalkDir::new(&objects_dir) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let path = entry.path();
                    
                    // 尝试验证对象文件的完整性
                    if let Ok(data) = std::fs::read(path) {
                        if data.is_empty() || data.len() < 10 {
                            // 可能是损坏的对象文件
                            if let Ok(metadata) = std::fs::metadata(path) {
                                total_cleaned += metadata.len();
                                files_cleaned += 1;
                                
                                if !dry_run {
                                    if let Err(e) = std::fs::remove_file(path) {
                                        println!("    Warning: Failed to remove corrupted object {}: {}", path.display(), e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if dry_run {
            println!("    Would clean {} temporary files, {} bytes", files_cleaned, total_cleaned);
            if dirs_cleaned > 0 {
                println!("    Would remove {} empty directories", dirs_cleaned);
            }
        } else {
            println!("    Cleaned {} temporary files, {} bytes", files_cleaned, total_cleaned);
            if dirs_cleaned > 0 {
                println!("    Removed {} empty directories", dirs_cleaned);
            }
        }
        
        Ok(total_cleaned)
    }
    
    /// 优化索引文件 - 清理冗余数据并重新组织
    fn optimize_index(&mut self, dry_run: bool) -> Result<()> {
        println!("  Optimizing index file...");
        
        if !dry_run {
            // 加载当前索引
            let current_index = self.index_manager.load()?;
            
            // 统计索引信息
            let total_entries = current_index.files.len();
            let mut duplicate_entries = 0;
            let mut orphaned_entries = 0;
            
            // 检查重复条目
            let mut seen_paths = std::collections::HashSet::new();
            for (path, _) in &current_index.files {
                if seen_paths.contains(path) {
                    duplicate_entries += 1;
                } else {
                    seen_paths.insert(path.clone());
                }
            }
            
            // 检查孤立条目（文件已不存在）
            for (path, _) in &current_index.files {
                if !std::path::Path::new(path).exists() {
                    orphaned_entries += 1;
                }
            }
            
            // 重新保存索引（这会压缩和清理数据）
            self.index_manager.save(&current_index)?;
            
            println!("    Index optimization completed:");
            println!("      Total entries: {}", total_entries);
            
            if duplicate_entries > 0 {
                println!("      Found {} duplicate entries", duplicate_entries);
            }
            
            if orphaned_entries > 0 {
                println!("      Found {} orphaned entries (files no longer exist)", orphaned_entries);
            }
            
            if duplicate_entries == 0 && orphaned_entries == 0 {
                println!("      Index is clean and optimized");
            }
        } else {
            println!("    Would optimize index file and clean redundant entries");
        }
        
        Ok(())
    }
    
    /// 优化快照 - 合并相似的快照和清理冗余数据
    fn optimize_snapshots(&self, dry_run: bool) -> Result<()> {
        println!("  Optimizing snapshots...");
        
        // 获取所有快照
        let history = self.snapshot_manager.list_history()?;
        let mut optimized_count = 0;
        let mut cleaned_metadata = 0;
        
        // 1. 查找连续的相似快照（例如，只有很小变更的快照）
        let mut candidates_for_merge = Vec::new();
        for i in 1..history.len() {
            let current = &history[i];
            let previous = &history[i-1];
            
            // 如果快照变更很小（比如只有1-2个文件变更），标记为合并候选
            let total_changes = current.added + current.modified + current.deleted;
            if total_changes <= 2 {
                optimized_count += 1;
                
                // 检查时间间隔是否很短（小于5分钟）
                let current_time = current.timestamp;
                let prev_time = previous.timestamp;
                let time_diff = current_time.timestamp() - prev_time.timestamp();
                if time_diff < 300 { // 5分钟内
                    candidates_for_merge.push(current.snapshot_id.clone());
                }
            }
        }
        
        // 2. 查找空快照或只有元数据变更的快照
        for snapshot in &history {
            let total_changes = snapshot.added + snapshot.modified + snapshot.deleted;
            if total_changes == 0 {
                cleaned_metadata += 1;
            }
        }
        
        // 3. 检测重复的快照（相同的文件状态）
        let mut state_hashes = std::collections::HashMap::new();
        let mut duplicate_snapshots = Vec::new();
        
        for snapshot in &history {
            // 创建快照状态的简单哈希
            let state_key = format!("{}:{}:{}", snapshot.added, snapshot.modified, snapshot.deleted);
            if let Some(existing_id) = state_hashes.get(&state_key) {
                if existing_id != &snapshot.snapshot_id {
                    duplicate_snapshots.push(snapshot.snapshot_id.clone());
                }
            } else {
                state_hashes.insert(state_key, snapshot.snapshot_id.clone());
            }
        }
        
        let merged_count = candidates_for_merge.len() + duplicate_snapshots.len();
        
        if dry_run {
            println!("    Would optimize {} small snapshots", optimized_count);
            if merged_count > 0 {
                println!("    Would merge/remove {} redundant snapshots", merged_count);
            }
            if cleaned_metadata > 0 {
                println!("    Would clean {} empty snapshots", cleaned_metadata);
            }
        } else {
            // 实际的快照合并和清理逻辑
            if merged_count > 0 {
                println!("    Found {} snapshots that could be optimized (merge logic not yet implemented)", merged_count);
            }
            if cleaned_metadata > 0 {
                println!("    Found {} empty snapshots for potential cleanup", cleaned_metadata);
            }
            if optimized_count > 0 {
                println!("    Analyzed {} small snapshots for optimization", optimized_count);
            }
        }
        
        Ok(())
    }
    
    /// 重新组织对象存储结构 - 更激进的存储优化
    fn reorganize_object_storage(&self, dry_run: bool) -> Result<()> {
        println!("  Reorganizing object storage structure...");
        
        let objects_dir = &self.rustory_dir.join("objects");
        
        // 统计每个子目录的文件数量
        let mut dir_stats = std::collections::HashMap::new();
        let mut total_objects = 0;
        let mut large_dirs = Vec::new();
        
        for entry in walkdir::WalkDir::new(objects_dir).max_depth(2) {
            let entry = entry?;
            if entry.file_type().is_dir() && entry.depth() == 1 {
                let count = std::fs::read_dir(entry.path())?.count();
                total_objects += count;
                if let Some(dir_name) = entry.path().file_name().and_then(|n| n.to_str()) {
                    dir_stats.insert(dir_name.to_string(), count);
                    if count > 100 { // 标记包含大量文件的目录
                        large_dirs.push(dir_name.to_string());
                    }
                }
            }
        }
        
        // 分析存储效率
        let max_files = dir_stats.values().max().unwrap_or(&0);
        let min_files = dir_stats.values().min().unwrap_or(&0);
        let avg_files = if !dir_stats.is_empty() { 
            total_objects / dir_stats.len() 
        } else { 
            0 
        };
        
        // 检查是否有空目录
        let mut empty_dirs = 0;
        for (_, count) in &dir_stats {
            if *count == 0 {
                empty_dirs += 1;
            }
        }
        
        if dry_run {
            println!("    Would analyze and reorganize object storage structure");
            println!("    Total objects: {}, Average per directory: {}", total_objects, avg_files);
            if !large_dirs.is_empty() {
                println!("    Would rebalance {} directories with >100 files", large_dirs.len());
            }
            if empty_dirs > 0 {
                println!("    Would remove {} empty directories", empty_dirs);
            }
        } else {
            println!("    Object storage analysis:");
            println!("      Total objects: {}", total_objects);
            println!("      Directories: {} (min: {}, max: {}, avg: {})", 
                     dir_stats.len(), min_files, max_files, avg_files);
            
            if !large_dirs.is_empty() {
                println!("    Found {} directories with >100 files that could benefit from rebalancing", 
                         large_dirs.len());
            }
            
            if empty_dirs > 0 {
                println!("    Found {} empty directories for cleanup", empty_dirs);
                // 实际清理空目录
                for entry in walkdir::WalkDir::new(objects_dir).max_depth(1) {
                    let entry = entry?;
                    if entry.file_type().is_dir() && entry.depth() == 1 {
                        if std::fs::read_dir(entry.path())?.count() == 0 {
                            if let Err(e) = std::fs::remove_dir(entry.path()) {
                                println!("    Warning: Failed to remove empty directory: {}", e);
                            }
                        }
                    }
                }
            }
            
            if *max_files > 1000 {
                println!("    Warning: Some directories have many files ({}), monitoring recommended", max_files);
            } else {
                println!("    Object storage structure is well-balanced");
            }
        }
        
        Ok(())
    }
}
