use std::path::Path;

pub fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn format_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

pub fn truncate_hash(hash: &str, len: usize) -> String {
    if hash.len() > len {
        hash[..len].to_string()
    } else {
        hash.to_string()
    }
}

pub fn create_backup_name() -> String {
    // 备份名使用UTC时间，避免文件名中的时区问题
    let now = chrono::Utc::now();
    format!(
        "rustory-rollback/backup-{}",
        now.format("%Y-%m-%dT%H-%M-%S")
    )
}
