use anyhow::Result;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct ObjectStore {
    objects_dir: PathBuf,
}

impl ObjectStore {
    pub fn new(objects_dir: PathBuf) -> Self {
        Self { objects_dir }
    }

    pub fn store_file(&mut self, file_path: &Path) -> Result<String> {
        let content = fs::read(file_path)?;
        self.store_content(&content)
    }

    pub fn store_content(&mut self, content: &[u8]) -> Result<String> {
        let mut hasher = Sha1::new();
        hasher.update(content);
        let hash = format!("{:x}", hasher.finalize());

        let object_path = self.get_object_path(&hash);
        
        // 如果对象已存在，不需要重复存储
        if !object_path.exists() {
            if let Some(parent) = object_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // 使用压缩存储
            let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
            encoder.write_all(content)?;
            let compressed = encoder.finish()?;
            
            fs::write(object_path, compressed)?;
        }

        Ok(hash)
    }

    pub fn get_content(&self, hash: &str) -> Result<Vec<u8>> {
        let object_path = self.get_object_path(hash);
        
        if !object_path.exists() {
            return Err(anyhow::anyhow!("Object {} not found", hash));
        }

        let compressed = fs::read(object_path)?;
        let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
        let mut content = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut content)?;
        
        Ok(content)
    }

    pub fn restore_file(&self, hash: &str, target_path: &Path) -> Result<()> {
        let content = self.get_content(hash)?;
        
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(target_path, content)?;
        Ok(())
    }

    fn get_object_path(&self, hash: &str) -> PathBuf {
        // 使用前两个字符作为子目录，避免单个目录文件过多
        let (prefix, suffix) = hash.split_at(2);
        self.objects_dir.join(prefix).join(suffix)
    }

    pub fn exists(&self, hash: &str) -> bool {
        self.get_object_path(hash).exists()
    }

    /// 获取所有存储的对象哈希值
    pub fn list_all_objects(&self) -> Result<Vec<String>> {
        let mut objects = Vec::new();
        if self.objects_dir.exists() {
            self.collect_objects_recursive(&self.objects_dir, &mut objects)?;
        }
        Ok(objects)
    }

    /// 递归收集对象目录中的所有对象
    fn collect_objects_recursive(&self, dir: &Path, objects: &mut Vec<String>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.collect_objects_recursive(&path, objects)?;
            } else if path.is_file() {
                // 从路径重构对象哈希
                if let Some(parent) = path.parent() {
                    if let (Some(prefix), Some(suffix)) = (
                        parent.file_name().and_then(|s| s.to_str()),
                        path.file_name().and_then(|s| s.to_str())
                    ) {
                        let hash = format!("{}{}", prefix, suffix);
                        objects.push(hash);
                    }
                }
            }
        }
        Ok(())
    }

    /// 删除指定的对象
    pub fn remove_object(&mut self, hash: &str) -> Result<u64> {
        let object_path = self.get_object_path(hash);
        if !object_path.exists() {
            return Ok(0);
        }

        let metadata = fs::metadata(&object_path)?;
        let size = metadata.len();
        
        fs::remove_file(&object_path)?;
        
        // 尝试删除空的父目录
        if let Some(parent) = object_path.parent() {
            let _ = fs::remove_dir(parent); // 忽略错误，可能不为空
        }
        
        Ok(size)
    }

    /// 获取对象的大小
    pub fn get_object_size(&self, hash: &str) -> Result<u64> {
        let object_path = self.get_object_path(hash);
        let metadata = fs::metadata(object_path)?;
        Ok(metadata.len())
    }
}
