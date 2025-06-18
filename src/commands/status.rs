use anyhow::Result;
use std::env;
use colored::*;

use crate::Repository;

pub struct StatusCommand;

impl StatusCommand {
    pub fn execute(verbose: bool, json: bool) -> Result<()> {
        let current_dir = env::current_dir()?;
        let root = Repository::find_root(&current_dir)?;
        let repo = Repository::new(root.clone())?;

        // 创建一个虚拟的忽略匹配器（现在在内部处理）
        let dummy_matcher = ignore::gitignore::GitignoreBuilder::new(&root).build()?;

        let (added, modified, deleted) = repo.index_manager
            .compare_with_current(&root, &dummy_matcher)?;

        if json {
            let status = serde_json::json!({
                "added": added,
                "modified": modified,
                "deleted": deleted,
                "clean": added.is_empty() && modified.is_empty() && deleted.is_empty()
            });
            println!("{}", serde_json::to_string_pretty(&status)?);
            return Ok(());
        }

        if added.is_empty() && modified.is_empty() && deleted.is_empty() {
            println!("{}", "Working directory clean".green());
            return Ok(());
        }

        // 显示状态摘要
        let total_changes = added.len() + modified.len() + deleted.len();
        println!("{} {} file(s) changed", "Status:".bold(), total_changes.to_string().yellow());
        
        if !added.is_empty() {
            println!("\n{} {}:", "Added".green().bold(), added.len());
            for path in &added {
                if verbose {
                    // 显示文件大小等详细信息
                    if let Ok(metadata) = std::fs::metadata(root.join(path)) {
                        println!("  {} {} ({})", "+".green(), path.display(), Self::format_size(metadata.len()));
                    } else {
                        println!("  {} {}", "+".green(), path.display());
                    }
                } else {
                    println!("  {} {}", "+".green(), path.display());
                }
            }
        }

        if !modified.is_empty() {
            println!("\n{} {}:", "Modified".yellow().bold(), modified.len());
            for path in &modified {
                if verbose {
                    if let Ok(metadata) = std::fs::metadata(root.join(path)) {
                        println!("  {} {} ({})", "~".yellow(), path.display(), Self::format_size(metadata.len()));
                    } else {
                        println!("  {} {}", "~".yellow(), path.display());
                    }
                } else {
                    println!("  {} {}", "~".yellow(), path.display());
                }
            }
        }

        if !deleted.is_empty() {
            println!("\n{} {}:", "Deleted".red().bold(), deleted.len());
            for path in &deleted {
                println!("  {} {}", "-".red(), path.display());
            }
        }

        // 显示提示信息
        println!("\n{}", "Tip: Use 'rustory commit -m \"message\"' to create a snapshot".dimmed());

        Ok(())
    }
    
    fn format_size(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / 1024.0 / 1024.0)
        } else {
            format!("{:.1} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
        }
    }
}
