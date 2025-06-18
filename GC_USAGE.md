# Rustory GC (垃圾回收) 功能

## 概述

Rustory 的垃圾回收 (GC) 功能可以帮助您清理不再需要的对象和快照，优化存储空间使用。

## 功能特性

### 1. 对象清理
- 自动识别不再被任何快照引用的对象
- 安全删除这些未引用的对象以释放存储空间
- 支持干运行模式，预览将要删除的内容

### 2. 快照清理
- 根据配置的保留策略删除过期的快照
- 支持基于时间和数量的保留策略
- 保持仓库历史的完整性

### 3. 自动垃圾回收
- 可配置为在每次提交后自动运行
- 智能触发机制，避免频繁执行

## 命令使用

### 基本用法

```bash
# 运行垃圾回收
rustory gc

# 干运行模式 - 预览但不实际删除
rustory gc --dry-run

# 包含快照清理
rustory gc --prune-expired

# 激进模式（预留给未来的优化功能）
rustory gc --aggressive
```

### 配置选项

```bash
# 设置保留天数（默认 30 天）
rustory config set gc_keep_days 14

# 设置保留快照数量（默认 50 个）
rustory config set gc_keep_snapshots 20

# 启用自动垃圾回收
rustory config set gc_auto_enabled true

# 查看当前配置
rustory config get gc_keep_days
rustory config get gc_keep_snapshots
rustory config get gc_auto_enabled
```

## 使用示例

### 示例 1：基本清理

```bash
# 查看当前状态
rustory status

# 运行干运行查看将要清理的内容
rustory gc --dry-run

# 实际运行清理
rustory gc
```

### 示例 2：配置保留策略

```bash
# 设置只保留最近 7 天的快照
rustory config set gc_keep_days 7

# 设置最多保留 10 个快照
rustory config set gc_keep_snapshots 10

# 运行清理包含过期快照
rustory gc --prune-expired
```

### 示例 3：启用自动清理

```bash
# 启用自动垃圾回收
rustory config set gc_auto_enabled true

# 创建新提交 - 会在适当时机自动触发 GC
rustory commit -m "New changes"
```

## 输出解读

垃圾回收运行后会显示以下信息：

```
Found 15 objects referenced by snapshots  # 被引用的对象数
Found 20 objects in storage              # 总存储对象数
Found 5 unreferenced objects             # 未引用的对象数
Found 3 snapshots to prune               # 要删除的快照数
Garbage collection completed:
  - Removed 5 unreferenced objects       # 实际删除的对象数
  - Freed 1024 bytes (1.00 MB)          # 释放的存储空间
```

## 安全性

- GC 操作是安全的，只会删除真正不再需要的对象
- 所有被当前快照引用的对象都会被保留
- 支持干运行模式，可以在实际执行前预览操作
- 快照删除遵循配置的保留策略，不会意外删除重要历史

## 最佳实践

1. **定期运行 GC**: 建议定期运行垃圾回收来优化存储空间
2. **使用干运行**: 在重要操作前使用 `--dry-run` 预览
3. **合理设置保留策略**: 根据项目需求设置合适的保留天数和快照数量
4. **启用自动清理**: 对于活跃的项目，建议启用自动垃圾回收
5. **监控存储使用**: 通过 GC 输出监控存储空间的使用情况

让我检查一下当前 GC 的默认配置设置： 根据代码可以看到，**GC 的自动功能默认是关闭的**：

- `gc_auto_enabled: false` - 自动 GC 默认关闭
- `gc_keep_days: Some(30)` - 默认保留 30 天
- `gc_keep_snapshots: Some(50)` - 默认保留 50 个快照

这意味着：

## 当前 GC 状态

### ❌ 自动 GC：默认关闭
- 用户需要手动启用：`rustory config set gc_auto_enabled true`
- 或者手动运行：`rustory gc`

### ✅ GC 配置：有默认值
- 保留策略已预设（30天/50个快照）
- 用户可以根据需要调整

让我验证一下这个行为： 现在测试提交时是否会自动运行 GC： 可以看到没有触发自动 GC。现在启用自动 GC 后再测试： ## 总结

**当前 GC 的默认行为：**

### 🔧 需要手动启用自动 GC
- **默认状态**：`gc_auto_enabled = false`
- **启用方法**：`rustory config set gc_auto_enabled true`
- **手动运行**：`rustory gc`

### 📋 已有默认配置
- **保留天数**：30 天
- **保留快照数**：50 个
- **用户可以调整**：通过 `rustory config set` 命令

### 🤔 设计考虑

这种设计有以下考虑：

**优点：**
- 用户可以主动控制是否启用自动清理
- 避免意外删除重要数据
- 新用户可以先熟悉工具再启用自动功能

**缺点：**
- 新用户可能不知道需要手动启用
- 可能导致存储空间持续增长

### 💡 建议改进

如果你希望提供更好的用户体验，我可以：

1. **修改默认值**：将 `gc_auto_enabled` 默认设为 `true`
2. **添加提示**：在存储空间较大时提示用户运行 GC
3. **智能提醒**：在特定条件下建议用户启用自动 GC

你希望我进行哪种改进？

## 注意事项

- 删除的对象和快照无法恢复，请谨慎操作
- 在团队环境中，建议协调 GC 操作的时机
- 大型仓库的 GC 操作可能需要一些时间
- 激进模式的额外优化功能尚未实现
