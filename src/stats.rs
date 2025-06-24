use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryStats {
    pub total_snapshots: usize,
    pub total_objects: usize,
    pub total_size_bytes: u64,
    pub compressed_size_bytes: u64,
    pub compression_ratio: f64,
    pub file_type_stats: HashMap<String, FileTypeStats>,
    pub timeline_stats: Vec<TimelineEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeStats {
    pub count: usize,
    pub total_size: u64,
    pub avg_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub date: String,
    pub commits: usize,
    pub files_changed: usize,
}

pub struct StatsCollector;

impl StatsCollector {
    pub fn collect_repository_stats(rustory_dir: &std::path::Path) -> Result<RepositoryStats> {
        let mut stats = RepositoryStats {
            total_snapshots: 0,
            total_objects: 0,
            total_size_bytes: 0,
            compressed_size_bytes: 0,
            compression_ratio: 0.0,
            file_type_stats: HashMap::new(),
            timeline_stats: Vec::new(),
        };

        // 统计快照数量
        let snapshots_dir = rustory_dir.join("snapshots");
        if snapshots_dir.exists() {
            stats.total_snapshots = std::fs::read_dir(snapshots_dir)?.count();
        }

        // 统计对象存储
        let objects_dir = rustory_dir.join("objects");
        if objects_dir.exists() {
            Self::collect_object_stats(&objects_dir, &mut stats)?;
        }

        // 计算压缩比
        if stats.total_size_bytes > 0 {
            stats.compression_ratio =
                stats.compressed_size_bytes as f64 / stats.total_size_bytes as f64;
        }

        Ok(stats)
    }

    fn collect_object_stats(
        objects_dir: &std::path::Path,
        stats: &mut RepositoryStats,
    ) -> Result<()> {
        for entry in walkdir::WalkDir::new(objects_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                stats.total_objects += 1;
                let metadata = entry.metadata()?;
                stats.compressed_size_bytes += metadata.len();

                // 尝试解压以获取原始大小
                if let Ok(original_size) = Self::get_original_object_size(entry.path()) {
                    stats.total_size_bytes += original_size;
                }
            }
        }
        Ok(())
    }

    fn get_original_object_size(path: &std::path::Path) -> Result<u64> {
        let compressed = std::fs::read(path)?;
        let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
        let mut content = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut content)?;
        Ok(content.len() as u64)
    }

    pub fn print_stats(stats: &RepositoryStats) {
        println!("📊 Repository Statistics");
        println!("========================");
        println!("Total snapshots: {}", stats.total_snapshots);
        println!("Total objects: {}", stats.total_objects);
        
        // 智能选择显示单位
        let (original_size_str, compressed_size_str, space_saved_str) = if stats.total_size_bytes < 1024 {
            // 小于1KB，显示字节
            (
                format!("{} bytes", stats.total_size_bytes),
                format!("{} bytes", stats.compressed_size_bytes),
                format!("{} bytes", stats.total_size_bytes as i64 - stats.compressed_size_bytes as i64),
            )
        } else if stats.total_size_bytes < 1024 * 1024 {
            // 小于1MB，显示KB
            (
                format!("{:.2} KB", stats.total_size_bytes as f64 / 1024.0),
                format!("{:.2} KB", stats.compressed_size_bytes as f64 / 1024.0),
                format!("{:.2} KB", (stats.total_size_bytes as i64 - stats.compressed_size_bytes as i64) as f64 / 1024.0),
            )
        } else {
            // 大于1MB，显示MB
            (
                format!("{:.2} MB", stats.total_size_bytes as f64 / 1024.0 / 1024.0),
                format!("{:.2} MB", stats.compressed_size_bytes as f64 / 1024.0 / 1024.0),
                format!("{:.2} MB", (stats.total_size_bytes as i64 - stats.compressed_size_bytes as i64) as f64 / 1024.0 / 1024.0),
            )
        };

        println!("Original size: {}", original_size_str);
        println!("Compressed size: {}", compressed_size_str);
        
        if stats.total_size_bytes > 0 {
            println!(
                "Compression ratio: {:.2}%",
                (stats.compressed_size_bytes as f64 / stats.total_size_bytes as f64) * 100.0
            );
        } else {
            println!("Compression ratio: N/A");
        }
        
        let space_saved_value = stats.total_size_bytes as i64 - stats.compressed_size_bytes as i64;
        if space_saved_value >= 0 {
            println!("Space saved: {}", space_saved_str);
        } else {
            println!("Space overhead: {}", space_saved_str.replace("-", ""));
        }
    }
}
