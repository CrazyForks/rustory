use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_output_format")]
    pub output_format: String,

    #[serde(default = "default_editor")]
    pub editor: String,

    #[serde(default = "default_max_file_size")]
    pub max_file_size_mb: u64,

    #[serde(default)]
    pub tags: HashMap<String, String>,

    #[serde(default)]
    pub backup_enabled: bool,

    // GC 相关配置
    #[serde(default)]
    pub gc_keep_days: Option<u32>,

    #[serde(default)]
    pub gc_keep_snapshots: Option<usize>,

    #[serde(default)]
    pub gc_auto_enabled: bool,

    // 时区显示设置
    #[serde(default = "default_use_local_timezone")]
    pub use_local_timezone: bool,
}

fn default_output_format() -> String {
    "table".to_string()
}

fn default_editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string())
}

fn default_max_file_size() -> u64 {
    100 // MB
}

fn default_use_local_timezone() -> bool {
    true // 默认使用本地时区
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_format: default_output_format(),
            editor: default_editor(),
            max_file_size_mb: default_max_file_size(),
            tags: HashMap::new(),
            backup_enabled: true,
            gc_keep_days: Some(30),
            gc_keep_snapshots: Some(50),
            gc_auto_enabled: false,
            use_local_timezone: default_use_local_timezone(),
        }
    }
}

impl Config {
    pub fn load(rustory_dir: &Path) -> Result<Self> {
        let config_path = rustory_dir.join("config.toml");
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, rustory_dir: &Path) -> Result<()> {
        let config_path = rustory_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "output_format" => Some(self.output_format.clone()),
            "editor" => Some(self.editor.clone()),
            "max_file_size_mb" => Some(self.max_file_size_mb.to_string()),
            "backup_enabled" => Some(self.backup_enabled.to_string()),
            "gc_keep_days" => self.gc_keep_days.map(|v| v.to_string()),
            "gc_keep_snapshots" => self.gc_keep_snapshots.map(|v| v.to_string()),
            "gc_auto_enabled" => Some(self.gc_auto_enabled.to_string()),
            "use_local_timezone" => Some(self.use_local_timezone.to_string()),
            _ => self.tags.get(key).cloned(),
        }
    }

    pub fn set(&mut self, key: &str, value: String) -> Result<()> {
        match key {
            "output_format" => self.output_format = value,
            "editor" => self.editor = value,
            "max_file_size_mb" => self.max_file_size_mb = value.parse()?,
            "backup_enabled" => self.backup_enabled = value.parse()?,
            "gc_keep_days" => self.gc_keep_days = Some(value.parse()?),
            "gc_keep_snapshots" => self.gc_keep_snapshots = Some(value.parse()?),
            "gc_auto_enabled" => self.gc_auto_enabled = value.parse()?,
            "use_local_timezone" => self.use_local_timezone = value.parse()?,
            _ => {
                self.tags.insert(key.to_string(), value);
            }
        }
        Ok(())
    }
}
