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
            let mut encoder =
                flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
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

    /// 获取对象的文件大小
    pub fn get_object_size(&self, hash: &str) -> Result<u64> {
        let object_path = self.get_object_path(hash);

        if !object_path.exists() {
            return Err(anyhow::anyhow!("Object {} not found", hash));
        }

        let metadata = std::fs::metadata(&object_path)?;
        Ok(metadata.len())
    }

    /// 列出所有对象的哈希值
    pub fn list_all_objects(&self) -> Result<Vec<String>> {
        let mut objects = Vec::new();

        for entry in walkdir::WalkDir::new(&self.objects_dir).max_depth(2) {
            let entry = entry?;
            if entry.file_type().is_file() && entry.depth() == 2 {
                if let Some(filename) = entry.path().file_name().and_then(|n| n.to_str()) {
                    if let Some(parent) = entry
                        .path()
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                    {
                        let hash = format!("{}{}", parent, filename);
                        objects.push(hash);
                    }
                }
            }
        }

        Ok(objects)
    }

    /// 删除对象
    pub fn remove_object(&self, hash: &str) -> Result<()> {
        let object_path = self.get_object_path(hash);

        if object_path.exists() {
            std::fs::remove_file(object_path)?;
        }

        Ok(())
    }

    /// 检查两个对象的内容是否相同
    pub fn objects_equal(&self, hash1: &str, hash2: &str) -> Result<bool> {
        if hash1 == hash2 {
            return Ok(true);
        }

        let content1 = self.get_content(hash1)?;
        let content2 = self.get_content(hash2)?;

        Ok(content1 == content2)
    }

    /// 重新压缩对象以获得更好的压缩比
    pub fn recompress_object(&mut self, hash: &str) -> Result<u64> {
        let object_path = self.get_object_path(hash);

        if !object_path.exists() {
            return Err(anyhow::anyhow!("Object {} not found", hash));
        }

        // 读取并解压现有内容
        let compressed = std::fs::read(&object_path)?;
        let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
        let mut content = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut content)?;

        // 使用更高级别的压缩重新压缩
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        std::io::Write::write_all(&mut encoder, &content)?;
        let new_compressed = encoder.finish()?;

        // 只有在新压缩文件更小时才替换
        if new_compressed.len() < compressed.len() {
            std::fs::write(&object_path, &new_compressed)?;
            Ok(new_compressed.len() as u64)
        } else {
            Ok(compressed.len() as u64)
        }
    }
}
