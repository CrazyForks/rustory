# Rustory

> 🚀 **轻量级本地版本管理工具** - 用 Rust 编写的高性能版本控制系统

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)](https://github.com/your-repo/rustory)

## ✨ 项目简介

Rustory 是一个基于 Rust 的版本控制工具，旨在帮助开发者轻松管理项目的快照、历史记录和配置。它提供了类似于 Git 的功能，但更专注于简单易用性。Rustory 是一款轻量级本地版本管理工具，支持 Linux、macOS 和 Windows 等多平台，无需依赖外部命令或脚本解释器，即可跟踪、快照和还原项目文件变更。

### 🎯 设计目标
- **本地优先**: 专为个人开发者和脚本作者设计，无需分布式协作
- **轻量高效**: 纯 Rust 实现，无外部依赖，启动快速
- **简单易用**: 直观的命令行界面，快速上手
- **存储优化**: 内容去重 + 压缩存储，节省磁盘空间

### 🏗️ 核心特性
- ✅ **快照管理**: 快速创建和恢复项目快照
- ✅ **差异比较**: 智能的文件差异检测和显示
- ✅ **标签系统**: 为重要快照添加描述性标签
- ✅ **忽略规则**: Git 风格的文件忽略模式
- ✅ **垃圾回收**: 自动清理过期数据，优化存储空间
- ✅ **完整性验证**: 数据完整性检查和修复
- ✅ **丰富统计**: 详细的仓库使用统计信息

## 📦 安装指南

### 前置要求
- **Rust 版本**: 1.70 或更高
- **操作系统**: Linux、macOS 或 Windows

### 安装步骤

1. **确保已安装 Rust 环境**
   ```bash
   # 安装 Rust (如果尚未安装)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **克隆并构建项目**
   ```bash
   git clone https://github.com/your-repo/rustory.git
   cd rustory
   cargo build --release
   ```

3. **安装到系统路径 (可选)**
   ```bash
   # Linux/macOS
   sudo cp target/release/rustory /usr/local/bin/
   
   # Windows - 添加到 PATH 环境变量
   copy target\release\rustory.exe C:\Windows\System32\
   ```

4. **验证安装**
   ```bash
   rustory --version
   ```

## 🏛️ 系统架构

### 存储结构
Rustory 在工作目录下创建 `.rustory` 文件夹，包含以下内容：

```
.rustory/
├── config.toml           # 用户配置：忽略规则、输出格式、备份策略等
├── ignore                # 忽略规则文件（Git 样式）
├── objects/              # 按 SHA-1 哈希存储内容
│   ├── ab/               # 使用哈希前两位作为子目录
│   │   └── cdef123...    # 压缩的文件内容
│   └── ...
├── index.json            # 当前工作区文件与哈希映射
├── history.log           # 快照日志：ID、时间、改动统计、备注
└── snapshots/            # 快照元数据 JSON 文件
    ├── abc123.json
    └── ...
```

### 核心概念

1. **对象存储**: 将文件内容写为二进制对象，文件名为其 SHA-1 哈希，实现内容去重
2. **索引管理**: 记录工作区文件路径与对应哈希，用于快速检测变更
3. **快照系统**: 保存一次索引状态，元数据存于 `snapshots/`，并记录在 `history.log`
4. **压缩存储**: 使用 gzip 压缩算法减少存储空间占用

### 存储优化
- **去重存储**: 相同内容的文件只存储一份
- **压缩算法**: 所有对象使用 gzip 压缩
- **目录分散**: 使用哈希前缀避免单目录文件过多
- **大文件限制**: 可配置的文件大小上限，默认 100MB

## 🚀 快速开始

### 初始化项目
```bash
# 在当前目录初始化
rustory init

# 在指定目录初始化
rustory init /path/to/project
```

### 基本工作流
```bash
# 1. 查看当前状态
rustory status

# 2. 创建快照
rustory commit -m "初始版本"

# 3. 查看历史
rustory history

# 4. 比较差异
rustory diff

# 5. 回滚更改
rustory rollback abc123
```

## 📋 命令详解

### 核心命令

#### `rustory init` - 初始化仓库
```bash
rustory init [path]
```
- **功能**: 创建新的 Rustory 仓库
- **参数**: `[path]` - 可选，指定初始化路径，默认当前目录
- **效果**: 创建 `.rustory` 目录结构，生成默认配置

#### `rustory commit` - 创建快照
```bash
rustory commit -m "提交信息" [--json]
```
- **功能**: 保存当前工作目录状态为新快照
- **参数**: 
  - `-m, --message <MSG>` - 快照描述信息
  - `--json` - 以 JSON 格式输出结果
- **示例**:
  ```bash
  rustory commit -m "修复解析器错误"
  # 输出: [snapshot ab12cd] 2025-06-18T15:30:00  added=2 modified=1 deleted=0
  ```

#### `rustory status` - 查看状态
```bash
rustory status [--verbose] [--json]
```
- **功能**: 显示工作目录相对于最新快照的变更
- **参数**:
  - `--verbose` - 显示详细信息（文件大小、修改时间）
  - `--json` - JSON 格式输出
- **示例输出**:
  ```
  已修改: src/lib.rs (1.2KB)
  已新增: tests/test_api.rs (0.8KB)
  已删除: docs/old.md
  ```

#### `rustory history` - 查看历史
```bash
rustory history [--json]
```
- **功能**: 显示所有快照的历史记录
- **示例输出**:
  ```
  ID       时间                     +  ~  -  消息
  ab12cd   2025-06-18T15:30:00      2  1  0  "修复解析器错误"
  ef34gh   2025-06-17T10:15:30      5  0  2  "添加新功能"
  ```

#### `rustory diff` - 比较差异
```bash
rustory diff [snapshot1] [snapshot2]
```
- **功能**: 显示文件差异
- **用法**:
  - 无参数: 当前状态与最新快照比较
  - 一个参数: 指定快照与当前状态比较
  - 两个参数: 两个快照之间比较
- **输出**: 彩色的行级差异显示

#### `rustory rollback` - 回滚更改
```bash
rustory rollback <snapshot_id> [--restore] [--keep-index]
```
- **功能**: 恢复到指定快照状态
- **参数**:
  - `<snapshot_id>` - 目标快照 ID 或标签
  - `--restore` - 直接恢复到工作目录（先备份当前状态）
  - `--keep-index` - 不更新索引文件
- **安全机制**: 默认导出到 `backup-<timestamp>/` 目录

### 管理命令

#### `rustory tag` - 标签管理
```bash
rustory tag <tag_name> <snapshot_id>
```
- **功能**: 为快照添加描述性标签
- **示例**: 
  ```bash
  rustory tag v1.0 ab12cd
  rustory rollback v1.0  # 使用标签回滚
  ```

#### `rustory ignore` - 忽略规则
```bash
rustory ignore [show|edit]
```
- **功能**: 管理文件忽略规则
- **规则格式**: 支持 Git 风格的 glob 模式
- **示例规则**:
  ```
  *.log
  temp/
  node_modules/
  target/
  ```

#### `rustory config` - 配置管理
```bash
rustory config get <key>           # 获取配置
rustory config set <key> <value>   # 设置配置
```
- **常用配置项**:
  - `output_format`: 输出格式 (table/json)
  - `max_file_size_mb`: 文件大小限制 (默认 100MB)
  - `gc_keep_days`: GC 保留天数 (默认 30 天)
  - `gc_keep_snapshots`: GC 保留快照数 (默认 50 个)
  - `gc_auto_enabled`: 自动 GC 开关 (默认 false)

### 工具命令

#### `rustory gc` - 垃圾回收
```bash
rustory gc [--dry-run] [--aggressive] [--prune-expired]
```
- **功能**: 清理不需要的对象和过期快照
- **参数**:
  - `--dry-run`: 预览模式，显示将删除的内容
  - `--aggressive`: 执行更激进的优化：
    - 对象重新压缩和重复检测
    - 深度碎片和临时文件清理
    - 索引文件优化和冗余条目清理
    - 快照分析和优化建议
    - 存储结构重组和空目录清理
  - `--prune-expired`: 包含过期快照清理
- **清理策略**: 基于配置的时间和数量限制

#### `rustory stats` - 统计信息
```bash
rustory stats [--json]
```
- **功能**: 显示仓库详细统计
- **包含信息**:
  - 仓库大小和压缩比
  - 文件类型分布
  - 快照数量和对象数量
  - 存储使用情况

#### `rustory verify` - 完整性验证
```bash
rustory verify [--fix]
```
- **功能**: 验证仓库数据完整性
- **检查项目**:
  - 快照文件格式验证
  - 对象文件可读性检查
  - 索引一致性验证
- **参数**: `--fix` - 尝试修复发现的问题

## 🔧 高级功能

### 垃圾回收策略

Rustory 提供灵活的垃圾回收机制来管理存储空间：

```bash
# 配置保留策略
rustory config set gc_keep_days 14      # 保留 14 天内的快照
rustory config set gc_keep_snapshots 20 # 最多保留 20 个快照

# 启用自动垃圾回收
rustory config set gc_auto_enabled true

# 手动运行垃圾回收
rustory gc --dry-run    # 预览清理内容
rustory gc              # 执行清理
```

### 批量操作

```bash
# 批量提交多个更改
find . -name "*.rs" -newer .rustory/index.json | rustory commit -m "批量更新"

# 基于模式的快照清理
rustory gc --prune-expired --pattern "temp-*"
```

### 配置优化

```bash
# 性能优化配置
rustory config set max_file_size_mb 50          # 限制大文件
rustory config set compression_level 6          # 调整压缩级别
rustory config set parallel_threads 4           # 并行处理线程数

# 输出格式配置
rustory config set output_format json           # 默认 JSON 输出
rustory config set colored_output true          # 彩色输出
```

## 🔍 故障排除

### 常见问题

#### 快照创建失败
```bash
# 检查磁盘空间
df -h .

# 检查文件权限
ls -la .rustory/

# 验证忽略规则
rustory ignore show

# 检查大文件
rustory status --verbose | grep "large"
```

#### 回滚冲突
```bash
# 保存当前工作后回滚
rustory commit -m "临时保存"
rustory rollback <target_snapshot>

# 或使用备份模式
rustory rollback <target_snapshot> --restore
```

#### 存储空间问题
```bash
# 检查仓库统计
rustory stats

# 运行垃圾回收
rustory gc --dry-run
rustory gc --prune-expired

# 清理大文件历史
rustory config set max_file_size_mb 10
rustory gc --aggressive
```

### 数据恢复

如果遇到数据损坏：

```bash
# 验证仓库完整性
rustory verify

# 尝试自动修复
rustory verify --fix

# 手动恢复（最后手段）
cp .rustory/snapshots/*.json backup/
rustory init --force
```

## 🚀 性能优化

### 存储优化建议

1. **定期垃圾回收**
   ```bash
   # 设置自动清理
   rustory config set gc_auto_enabled true
   rustory config set gc_keep_days 30
   ```

2. **文件大小限制**
   ```bash
   # 避免跟踪大文件
   rustory config set max_file_size_mb 50
   ```

3. **忽略规则优化**
   ```bash
   # 排除构建产物和临时文件
   echo "target/" >> .rustory/ignore
   echo "*.tmp" >> .rustory/ignore
   echo "node_modules/" >> .rustory/ignore
   ```

### 性能监控

```bash
# 查看操作耗时
time rustory commit -m "性能测试"

# 监控存储使用
rustory stats | grep "Size"

# 检查压缩效率
rustory stats | grep "Compression"
```

## 🛠️ 集成与扩展

### 编辑器集成

#### VS Code
```json
// settings.json
{
  "rustory.autoCommit": true,
  "rustory.commitInterval": 3600,
  "rustory.showStatus": true
}
```

#### Vim
```vim
" .vimrc
autocmd BufWritePost * silent! !rustory commit -m "Auto save"
```

### CI/CD 集成

#### GitHub Actions
```yaml
name: Rustory Snapshot
on: [push]
jobs:
  snapshot:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Create snapshot
        run: |
          rustory init
          rustory commit -m "CI Build ${{ github.run_number }}"
```

#### Shell 脚本集成
```bash
#!/bin/bash
# 自动化部署脚本
set -e

echo "创建部署前快照..."
rustory commit -m "Pre-deploy snapshot $(date)"

echo "执行部署..."
./deploy.sh

echo "创建部署后快照..."
rustory commit -m "Post-deploy snapshot $(date)"
```

## 🎯 与其他工具对比

| 功能 | Rustory | Git | SVN |
|------|---------|-----|-----|
| 本地优先 | ✅ | ⚠️ | ❌ |
| 启动速度 | ✅ | ⚠️ | ❌ |
| 存储效率 | ✅ | ✅ | ⚠️ |
| 分布式 | ❌ | ✅ | ❌ |
| 学习成本 | ✅ | ⚠️ | ⚠️ |
| 二进制文件 | ✅ | ⚠️ | ✅ |

### 适用场景

**推荐使用 Rustory**:
- 个人项目快照管理
- 脚本和配置文件版本控制
- 快速原型开发
- 文档和笔记版本管理
- 不需要远程协作的项目

**推荐使用 Git**:
- 团队协作开发
- 开源项目
- 复杂的分支管理需求
- 与 GitHub/GitLab 等平台集成

## 📈 项目路线图

### 当前版本 (v0.1.0)
- ✅ 核心版本控制功能
- ✅ 基础存储优化
- ✅ 垃圾回收机制
- ✅ 配置系统

### 下一版本 (v0.2.0)
- 🚧 并行处理优化
- 🚧 增量备份功能

### 未来版本
- 📋 同步支持
- 📋 API 接口
- 📋 插件系统基础



### 开发环境设置
```bash
# 克隆仓库
git clone https://github.com/uselibrary/rustory.git
cd rustory

# 安装开发依赖
cargo install cargo-watch
cargo install cargo-tarpaulin

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 代码规范
- 使用 `rustfmt` 格式化代码
- 通过 `clippy` 检查代码质量
- 为新功能添加测试
- 更新相关文档

## 📄 许可证

本项目采用 [GNU General Public License v3.0](LICENSE) 许可证。

## 🙏 致谢

感谢以下优秀的 Rust 库使项目成为可能：
- [clap](https://crates.io/crates/clap) - 命令行参数解析
- [serde](https://crates.io/crates/serde) - 序列化/反序列化
- [walkdir](https://crates.io/crates/walkdir) - 目录遍历
- [flate2](https://crates.io/crates/flate2) - 压缩算法
- [sha1](https://crates.io/crates/sha1) - 哈希计算
- [chrono](https://crates.io/crates/chrono) - 时间处理
- [colored](https://crates.io/crates/colored) - 彩色输出

---

<div align="center">

**[⬆ 回到顶部](#rustory)**

Made with ❤️ by the Rustory Teamnew line
