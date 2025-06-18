use anyhow::Result;
use std::env;

use crate::{Repository, security::SecurityManager};

pub struct VerifyCommand;

impl VerifyCommand {
    pub fn execute(fix: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root.clone())?;
        
        println!("🔍 Verifying repository integrity...");
        
        // 检查对象存储一致性
        let corrupted_objects = SecurityManager::verify_object_consistency(&repo.rustory_dir.join("objects"))?;
        
        if corrupted_objects.is_empty() {
            println!("✅ All objects are consistent");
        } else {
            println!("❌ Found {} corrupted objects:", corrupted_objects.len());
            for obj in &corrupted_objects {
                println!("  - {}", obj);
            }
            
            if fix {
                println!("🔧 Attempting to repair...");
                // 这里可以实现修复逻辑
                println!("Repair functionality not yet implemented");
            }
        }
        
        // 验证所有快照
        let snapshots_dir = repo.rustory_dir.join("snapshots");
        let mut verified_snapshots = 0;
        let mut failed_snapshots = 0;
        
        if snapshots_dir.exists() {
            for entry in std::fs::read_dir(&snapshots_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    match SecurityManager::verify_snapshot_integrity(&path) {
                        Ok(_) => verified_snapshots += 1,
                        Err(_) => {
                            failed_snapshots += 1;
                            println!("❌ Snapshot verification failed: {}", path.display());
                        }
                    }
                }
            }
        }
        
        println!("\n📊 Verification Summary:");
        println!("  Verified snapshots: {}", verified_snapshots);
        println!("  Failed snapshots: {}", failed_snapshots);
        println!("  Corrupted objects: {}", corrupted_objects.len());
        
        if failed_snapshots == 0 && corrupted_objects.is_empty() {
            println!("✅ Repository is healthy!");
        } else {
            println!("⚠️  Repository has integrity issues that need attention");
        }
        
        Ok(())
    }
}
