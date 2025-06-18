use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod commands;
pub mod config;
pub mod diff_engine;
pub mod index;
pub mod objects;
pub mod repository;
pub mod security;
pub mod snapshot;
pub mod stats;
pub mod utils;
pub mod workflow;

pub use repository::Repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub hash: String,
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub files: HashMap<PathBuf, FileEntry>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message: String,
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
    pub files: HashMap<PathBuf, FileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub snapshot_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
    pub message: String,
}
