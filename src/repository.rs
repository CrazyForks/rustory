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
        self.snapshot_manager.create_snapshot(
            &self.root,
            &self.config,
            &mut self.object_store,
            &mut self.index_manager,
            message,
        )
    }
}
