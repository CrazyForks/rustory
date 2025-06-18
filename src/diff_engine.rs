use anyhow::Result;
use std::collections::HashMap;

pub struct DiffEngine;

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub content: String,
    pub line_number: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffLineType {
    Added,
    Removed,
    Modified,
    Context,
}

impl DiffEngine {
    /// 生成两个文本内容之间的差异
    pub fn diff_text(old_content: &str, new_content: &str) -> Vec<DiffLine> {
        let old_lines: Vec<&str> = old_content.lines().collect();
        let new_lines: Vec<&str> = new_content.lines().collect();
        
        Self::myers_diff(&old_lines, &new_lines)
    }
    
    /// 使用Myers差异算法生成差异
    fn myers_diff(old_lines: &[&str], new_lines: &[&str]) -> Vec<DiffLine> {
        let mut result = Vec::new();
        let mut old_idx = 0;
        let mut new_idx = 0;
        
        // 简化的差异算法 - 实际项目中可以使用更复杂的算法
        while old_idx < old_lines.len() || new_idx < new_lines.len() {
            if old_idx >= old_lines.len() {
                // 只剩新行
                result.push(DiffLine {
                    line_type: DiffLineType::Added,
                    content: format!("+{}", new_lines[new_idx]),
                    line_number: Some(new_idx + 1),
                });
                new_idx += 1;
            } else if new_idx >= new_lines.len() {
                // 只剩旧行
                result.push(DiffLine {
                    line_type: DiffLineType::Removed,
                    content: format!("-{}", old_lines[old_idx]),
                    line_number: Some(old_idx + 1),
                });
                old_idx += 1;
            } else if old_lines[old_idx] == new_lines[new_idx] {
                // 相同行
                result.push(DiffLine {
                    line_type: DiffLineType::Context,
                    content: format!(" {}", old_lines[old_idx]),
                    line_number: Some(old_idx + 1),
                });
                old_idx += 1;
                new_idx += 1;
            } else {
                // 不同行 - 简单处理为删除+添加
                result.push(DiffLine {
                    line_type: DiffLineType::Removed,
                    content: format!("-{}", old_lines[old_idx]),
                    line_number: Some(old_idx + 1),
                });
                result.push(DiffLine {
                    line_type: DiffLineType::Added,
                    content: format!("+{}", new_lines[new_idx]),
                    line_number: Some(new_idx + 1),
                });
                old_idx += 1;
                new_idx += 1;
            }
        }
        
        result
    }
    
    /// 打印差异到控制台
    pub fn print_diff(diff_lines: &[DiffLine], file_path: &str) {
        use colored::*;
        
        println!("{}", format!("--- {}", file_path).bold());
        println!("{}", format!("+++ {}", file_path).bold());
        
        for line in diff_lines {
            match line.line_type {
                DiffLineType::Added => println!("{}", line.content.green()),
                DiffLineType::Removed => println!("{}", line.content.red()),
                DiffLineType::Modified => println!("{}", line.content.yellow()),
                DiffLineType::Context => println!("{}", line.content.dimmed()),
            }
        }
    }
    
    /// 生成统计信息
    pub fn get_diff_stats(diff_lines: &[DiffLine]) -> (usize, usize, usize) {
        let mut added = 0;
        let mut removed = 0;
        let mut modified = 0;
        
        for line in diff_lines {
            match line.line_type {
                DiffLineType::Added => added += 1,
                DiffLineType::Removed => removed += 1,
                DiffLineType::Modified => modified += 1,
                DiffLineType::Context => {}
            }
        }
        
        (added, removed, modified)
    }
}
