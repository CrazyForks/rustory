use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{FileEntry, Index};

pub struct IndexManager {
    index_path: PathBuf,
}

impl IndexManager {
    pub fn new(index_path: PathBuf) -> Self {
        Self { index_path }
    }

    pub fn load(&self) -> Result<Index> {
        if self.index_path.exists() {
            let content = fs::read_to_string(&self.index_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Index::new())
        }
    }

    pub fn save(&self, index: &Index) -> Result<()> {
        let content = serde_json::to_string_pretty(index)?;
        fs::write(&self.index_path, content)?;
        Ok(())
    }

    pub fn scan_directory(
        &self,
        root: &Path,
        _ignore_matcher: &ignore::gitignore::Gitignore,
    ) -> Result<Index> {
        let mut index = Index::new();

        // 使用 ignore::WalkBuilder 来正确处理忽略规则
        let mut builder = ignore::WalkBuilder::new(root);
        builder.hidden(false); // 显示隐藏文件，但会应用 .rustory/ignore 规则
        
        // 添加自定义忽略文件
        let ignore_path = root.join(".rustory/ignore");
        if ignore_path.exists() {
            builder.add_custom_ignore_filename(".rustory/ignore");
        }

        for entry in builder.build() {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let relative_path = path.strip_prefix(root)?;
                
                // 显式跳过 .rustory 目录和 rustory-rollback 目录
                if relative_path.starts_with(".rustory") || relative_path.starts_with("rustory-rollback") {
                    continue;
                }

                let metadata = entry.metadata()?;
                let size = metadata.len();
                let modified = chrono::DateTime::from(metadata.modified()?);

                // 计算文件哈希
                let content = fs::read(path)?;
                let hash = {
                    use sha1::{Digest, Sha1};
                    let mut hasher = Sha1::new();
                    hasher.update(&content);
                    format!("{:x}", hasher.finalize())
                };

                index.files.insert(
                    relative_path.to_path_buf(),
                    FileEntry {
                        path: relative_path.to_path_buf(),
                        hash,
                        size,
                        modified,
                    },
                );
            }
        }

        Ok(index)
    }

    pub fn compare_with_current(
        &self,
        root: &Path, 
        ignore_matcher: &ignore::gitignore::Gitignore,
    ) -> Result<(Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>)> {
        let old_index = self.load()?;
        let new_index = self.scan_directory(root, ignore_matcher)?;

        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut deleted = Vec::new();

        // 检查新增和修改的文件
        for (path, new_entry) in &new_index.files {
            match old_index.files.get(path) {
                None => added.push(path.clone()),
                Some(old_entry) => {
                    if old_entry.hash != new_entry.hash {
                        modified.push(path.clone());
                    }
                }
            }
        }

        // 检查删除的文件
        for path in old_index.files.keys() {
            if !new_index.files.contains_key(path) {
                deleted.push(path.clone());
            }
        }

        Ok((added, modified, deleted))
    }
}
