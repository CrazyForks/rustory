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
  - `--aggressive`: 更激进的优化（预留功能）
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
- 🚧 Web UI 界面
- 🚧 插件系统基础

### 未来版本
- 📋 远程同步支持
- 📋 简单分支管理
- 📋 API 接口
- 📋 图形化客户端

## 🤝 贡献指南

我们欢迎社区贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

### 开发环境设置
```bash
# 克隆仓库
git clone https://github.com/your-repo/rustory.git
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

## 📞 支持与反馈

- **问题报告**: [GitHub Issues](https://github.com/your-repo/rustory/issues)
- **功能请求**: [GitHub Discussions](https://github.com/your-repo/rustory/discussions)
- **邮件联系**: rustory@example.com

---

<div align="center">

**[⬆ 回到顶部](#rustory)**

Made with ❤️ by the Rustory Team

</div>

#### 查看历史记录
查看项目的历史快照记录：
```bash
rustory history
```

参数：
- `--json`：JSON 格式输出

示例输出：
```
ID       时间                     +  ~  -  消息
ab12cd   2025-06-17T15:30:00      2  1  0  "修复解析器错误"
ef34gh   2025-06-16T10:15:30      5  0  2  "添加新功能"
```

#### 比较差异
比较两个快照之间或与当前工作目录的差异：
```bash
rustory diff [快照ID1] [快照ID2]
```

- 无参数：当前状态与最近快照比较
- 一个参数：指定快照与当前状态比较
- 两个参数：两个快照间比较

#### 回滚更改
回滚到指定的快照：
```bash
rustory rollback <快照ID> [--restore] [--keep-index]
```

参数：
- `--restore`：直接恢复到工作目录（会先备份当前状态）
- `--keep-index`：不更新索引文件

默认情况下，回滚会将快照内容导出到 `backup-<时间戳>/` 目录，不直接修改工作目录。

#### 标签管理
为快照添加描述性标签：
```bash
rustory tag <标签名> <快照ID>
```

使用标签代替 ID：
```bash
rustory rollback v1.0    # 使用标签回滚
```

#### 忽略文件
管理忽略文件规则：
```bash
rustory ignore [show|edit]
```
- `show`：显示当前忽略规则
- `edit`：打开编辑器修改忽略规则

忽略规则与 Git 类似，例如：
```
*.log
temp/
node_modules/
```

#### 配置项目
设置或查看项目配置：
```bash
rustory config get <key>           # 获取配置
rustory config set <key> <value>   # 设置配置
```

常用配置项：
- `output_format`：输出格式（table/json）
- `editor`：默认编辑器
- `max_file_size_mb`：最大文件大小限制（MB）
- `backup_enabled`：是否启用备份
- `gc_keep_days`：垃圾回收保留天数
- `gc_keep_snapshots`：垃圾回收保留快照数量
- `gc_auto_enabled`：是否启用自动垃圾回收

#### 垃圾回收
清理不再需要的对象和快照，优化存储空间：
```bash
rustory gc                    # 运行垃圾回收
rustory gc --dry-run          # 预览模式，显示将要删除的内容
rustory gc --prune-expired    # 包含过期快照清理
rustory gc --aggressive       # 激进模式（更多优化）
```

垃圾回收功能：
- **对象清理**：删除不再被任何快照引用的对象
- **快照清理**：根据配置删除过期的快照
- **自动清理**：可配置为在提交后自动运行
- **安全性**：提供干运行模式预览操作

配置垃圾回收策略：
```bash
# 设置保留最近 14 天的快照
rustory config set gc_keep_days 14

# 设置最多保留 20 个快照
rustory config set gc_keep_snapshots 20

# 启用自动垃圾回收
rustory config set gc_auto_enabled true
```

## 高级功能

### 导出快照
导出特定快照为压缩包：
```bash
rustory export <快照ID> [输出路径]
```

### 批量标签
为多个快照批量添加标签：
```bash
rustory batch-tag --pattern "v*.0" --days 7
```

## 常见问题解决

### 快照创建失败
1. 检查磁盘空间是否充足
2. 确认文件权限是否正确
3. 验证忽略规则是否正确配置

```bash
# 检查忽略规则
rustory ignore show

# 验证状态
rustory status
```

### 回滚冲突
回滚时发现工作区有未提交的更改：

```bash
# 先创建新快照保存当前工作
rustory commit -m "保存临时工作"

# 然后回滚
rustory rollback <快照ID>
```

## 与其他工具集成

### 编辑器集成
Rustory 可以与 VS Code、Vim 等编辑器集成：

```bash
# VS Code 设置（settings.json）
{
  "rustory.autoCommit": true,
  "rustory.commitInterval": 3600
}
```

### CI/CD 工作流
在 CI/CD 流程中自动创建快照：

```yaml
# 示例 GitHub Actions 工作流
jobs:
  build:
    steps:
      - uses: actions/checkout@v3
      - name: Create snapshot
        run: |
          rustory commit -m "CI Build ${{ github.run_number }}"
```


### 代码规范
- 代码格式：使用 `rustfmt`
- 代码质量：通过 `clippy` 检查
- 测试覆盖：为新功能添加测试

## 性能优化建议
1. **定期垃圾回收**：定期运行 `rustory gc` 清理不需要的对象和过期快照
2. **配置保留策略**：根据项目需要合理设置快照保留天数和数量
3. **启用自动清理**：设置 `gc_auto_enabled=true` 让系统自动维护存储空间
4. **调整文件大小限制**：通过 `max_file_size_mb` 避免跟踪过大的文件
5. **优化忽略规则**：排除大型二进制文件、构建产物和临时文件

要回答您的问题，我需要了解项目的设计和实现方式。让我先查看一些关键文件来理解这个 Rust 项目的设计。 让我继续查看框架设计和实现文件，尤其是关于存储实现的部分： 现在我需要了解一下项目的核心实现文件，特别是与存储相关的部分。让我们首先了解对象存储结构的实现： 接下来看一下快照相关的实现： 让我继续查看快照文件的后半部分： 让我看一下配置文件的设置，这可能包含一些与存储相关的选项： 最后，让我看一下 `index.rs` 来了解索引的存储实现： 让我看一下索引文件的后半部分： 最后让我看一下整体的项目结构和主要数据结构： 基于以上代码分析，我可以回答您关于硬盘占用的问题了：

## 关于硬盘占用的分析

### 存储优化策略

1. **去重存储**：`Rustory` 使用了内容寻址存储系统，通过 `SHA-1` 哈希将文件内容唯一标识，相同内容的文件只会存储一份，显著减少了存储空间占用。

2. **压缩存储**：在 `objects.rs` 中可以看到，`Rustory` 使用了 `GzEncoder` 对文件内容进行压缩后再存储，减少了磁盘空间的使用：
   ```rust
   let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
   encoder.write_all(content)?;
   let compressed = encoder.finish()?;
   ```

3. **目录分散**：使用哈希前两位作为子目录名，避免了单个目录下文件过多导致的性能问题：
   ```rust
   fn get_object_path(&self, hash: &str) -> PathBuf {
       // 使用前两个字符作为子目录，避免单个目录文件过多
       let (prefix, suffix) = hash.split_at(2);
       self.objects_dir.join(prefix).join(suffix)
   }
   ```

4. **文件大小限制**：在配置中设置了文件大小上限（默认为 100MB），并在创建快照时跳过大文件：
   ```rust
   if entry.size > config.max_file_size_mb * 1024 * 1024 {
       eprintln!("Warning: Skipping large file: {} ({}MB)", 
           path.display(), entry.size / 1024 / 1024);
       continue;
   }
   ```

### 潜在的硬盘占用问题

1. **历史快照累积**：每次创建快照时，系统会保存所有被跟踪文件的内容。虽然使用了去重和压缩，但如果项目频繁更改且文件较大，可能会导致对象存储逐渐增大。当前设计中没有实现垃圾收集或历史裁剪机制。

2. **缺少垃圾收集**：在代码中我没有看到对不再被任何快照引用的对象进行清理的机制。这可能导致随着时间的推移，被删除文件的历史版本会继续占用空间。

3. **元数据冗余**：每个快照会保存完整的文件映射，当项目文件数量非常多时，这些元数据本身也会占用一定的空间。

### 与其他版本控制系统的对比

相比 `Git`，`Rustory` 的存储模型基本相似，都使用了内容寻址和去重，但有几个关键差异：

1. `Git` 有垃圾收集机制（`git gc`），而当前 `Rustory` 无类似功能。
2. `Rustory` 默认使用压缩存储所有对象，而 `Git` 只对某些对象使用压缩。
3. `Rustory` 有文件大小上限设置，避免了意外提交大文件导致的存储爆炸。

### 后期优化方向

1. **实现垃圾收集**：添加一个清理命令，删除不再被任何快照引用的对象。

2. **快照裁剪**：提供删除旧快照的功能，或合并连续的快照以减少历史记录。

3. **选择性存储**：对于特定类型的大文件，可以只存储元数据或差异，而非完整内容。

4. **存储配额**：为 `.rustory` 目录设置最大容量限制，并在接近限制时提醒用户。

5. **稀疏快照**：仅保存关键节点的完整快照，中间快照只存储差异。

### 结论

总体来看，`Rustory` 的设计采用了内容去重和压缩存储等优化策略，一般情况下不会导致极端的硬盘占用。但对于长期维护的大型项目，由于缺乏垃圾收集机制，可能会随着时间推移出现存储空间增长问题。通过实现垃圾收集、快照裁剪等功能，可以更有效地控制存储空间占用。


## 版本计划
- **0.2.0**: 增强差异比较功能
- **1.0.0**: API 稳定，完整文档

## 许可证
本项目采用 GPLv3 许可证。详见 [LICENSE](LICENSE) 文件。