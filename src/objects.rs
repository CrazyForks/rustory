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
}
