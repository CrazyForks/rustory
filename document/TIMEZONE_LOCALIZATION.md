# 时区本地化功能

Rustory 现在支持时区本地化显示，用户可以选择在查看历史记录时显示本地时间还是 UTC 时间。

## 特性

- **存储**：所有时间戳仍然以 UTC 时间存储，确保跨时区的一致性
- **显示**：可以配置是否将时间转换为本地时区显示
- **向后兼容**：默认启用本地时区显示，现有用户体验更好

## 配置

### 查看当前时区设置

```bash
rustory config get use_local_timezone
```

### 启用本地时区显示（默认）

```bash
rustory config set use_local_timezone true
```

### 使用 UTC 时间显示

```bash
rustory config set use_local_timezone false
```

## 时间格式

### 本地时区格式
```
2025-06-18 15:30:45
```

### UTC 时间格式
```
2025-06-18T15:30:45Z
```

## 示例

### 历史记录显示（本地时区）
```bash
$ rustory history
ID       Time                 +  ~  - Message
------------------------------------------------------------
abc12345 2025-06-18 15:30:45  3  2  1 "Initial commit"
def67890 2025-06-18 14:20:30  0  1  0 "Update documentation"
```

### 历史记录显示（UTC 时间）
```bash
$ rustory config set use_local_timezone false
$ rustory history
ID       Time                   +  ~  - Message
------------------------------------------------------------
abc12345 2025-06-18T07:30:45Z   3  2  1 "Initial commit"
def67890 2025-06-18T06:20:30Z   0  1  0 "Update documentation"
```

## 技术细节

- 存储：使用 `chrono::DateTime<chrono::Utc>` 确保时间的准确性
- 显示：使用 `chrono::DateTime::with_timezone(&chrono::Local)` 转换为本地时间
- 配置：存储在 `.rustory/config.toml` 中的 `use_local_timezone` 字段
- JSON 输出：始终使用 UTC 时间，保持 API 的一致性

## 注意事项

1. **存储不变**：改变显示设置不会影响已存储的快照数据
2. **JSON 输出**：使用 `--json` 参数时，时间戳始终以 UTC 格式输出
3. **文件名**：备份文件名仍使用 UTC 时间，避免文件系统兼容性问题
4. **跨平台**：自动检测系统时区，无需手动配置
