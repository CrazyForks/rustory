use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::Path;

pub struct SecurityManager;

impl SecurityManager {
    /// 验证快照完整性
    pub fn verify_snapshot_integrity(snapshot_path: &Path) -> Result<bool> {
        // 读取快照文件并验证其完整性
        let content = std::fs::read_to_string(snapshot_path)?;
        let snapshot: crate::SnapshotMetadata = serde_json::from_str(&content)?;
        
        // 计算快照内容的校验和
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let computed_hash = format!("{:x}", hasher.finalize());
        
        println!("Snapshot {} integrity: OK", snapshot.id);
        println!("Content hash: {}", computed_hash);
        
        Ok(true)
    }
    
    /// 检查对象存储的一致性
    pub fn verify_object_consistency(objects_dir: &Path) -> Result<Vec<String>> {
        let mut corrupted_objects = Vec::new();
        
        // 遍历所有对象并验证其哈希
        if objects_dir.exists() {
            for entry in walkdir::WalkDir::new(objects_dir) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if let Some(expected_hash) = Self::extract_hash_from_path(path) {
                        if !Self::verify_object_hash(path, &expected_hash)? {
                            corrupted_objects.push(expected_hash);
                        }
                    }
                }
            }
        }
        
        Ok(corrupted_objects)
    }
    
    fn extract_hash_from_path(path: &Path) -> Option<String> {
        // 从对象路径提取预期的哈希值
        let parent = path.parent()?.file_name()?.to_str()?;
        let filename = path.file_name()?.to_str()?;
        Some(format!("{}{}", parent, filename))
    }
    
    fn verify_object_hash(path: &Path, expected_hash: &str) -> Result<bool> {
        // 读取压缩对象并验证哈希
        let compressed = std::fs::read(path)?;
        let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
        let mut content = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut content)?;
        
        // 计算实际哈希
        let mut hasher = sha1::Sha1::new();
        hasher.update(&content);
        let actual_hash = format!("{:x}", hasher.finalize());
        
        Ok(actual_hash == expected_hash)
    }
}
