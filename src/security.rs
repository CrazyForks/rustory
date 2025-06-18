use anyhow::Result;
use std::path::Path;

pub struct SecurityManager;

impl SecurityManager {
    /// 简化的快照验证 - 仅检查文件是否为有效JSON
    pub fn verify_snapshot_integrity(snapshot_path: &Path) -> Result<bool> {
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
    pub fn verify_object_consistency(objects_dir: &Path) -> Result<Vec<String>> {
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
}
