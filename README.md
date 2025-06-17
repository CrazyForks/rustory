### 项目简介
Rustory 是一个基于 Rust 的版本控制工具，旨在帮助开发者轻松管理项目的快照、历史记录和配置。它提供了类似于 Git 的功能，但更专注于简单易用性。Rustory 是一款轻量级本地版本管理工具，支持 Linux、macOS 和 Windows 等多平台，无需依赖外部命令或脚本解释器，即可跟踪、快照和还原项目文件变更。

### 安装要求
- Rust 版本：1.70 或更高
- 操作系统：Linux、macOS 或 Windows

### 安装步骤
1. 确保已安装 [Rust](https://www.rust-lang.org/) 环境。
2. 克隆项目：
   ```bash
   git clone https://github.com/your-repo/rustory.git
   cd rustory
   ```
3. 构建项目：
   ```bash
   cargo build --release
   ```
4. 添加到路径（可选）：
   ```bash
   # Linux/macOS
   cp target/release/rustory /usr/local/bin/

   # Windows - 添加到 PATH 环境变量
   ```

## 系统架构

### 存储结构
Rustory 在工作目录下创建 `.rustory` 文件夹，包含以下内容：

```
.rustory/
├── config.toml           # 用户配置：忽略规则、输出格式、备份策略等
├── ignore                # 忽略规则文件（Git 样式）
├── objects/              # 按 SHA-1 哈希存储内容
├── index.json            # 当前工作区文件与哈希映射
├── history.log           # 快照日志：ID、时间、改动统计、备注
└── snapshots/            # 快照元数据 JSON 文件
```

### 核心概念
1. **对象存储**：将文件内容写为二进制对象，文件名为其 SHA-1 哈希，实现去重。
2. **索引**：记录工作区文件路径与对应哈希，用于检测变更。
3. **快照**：保存一次索引状态，元数据存于 `snapshots/`，并记录在 `history.log`。

## 使用方法

### 命令概览
Rustory 提供以下主要命令：

```
init     - 初始化新仓库
commit   - 创建新快照记录更改
status   - 显示当前工作目录状态
history  - 显示快照历史记录
diff     - 比较快照或工作目录的差异
rollback - 回滚到之前的快照
tag      - 为快照添加标签
ignore   - 管理忽略规则
config   - 管理配置选项
```

### 详细使用指南

#### 初始化项目
初始化一个新的 Rustory 项目：
```bash
rustory init [path]
```
- `[path]`：可选，指定初始化的路径，默认为当前目录

初始化会创建 `.rustory` 目录结构，生成默认配置，并建立首个索引。

#### 提交更改
提交当前工作目录的更改，创建新的快照：
```bash
rustory commit -m "提交信息"
```
参数：
- `-m, --message <MSG>`：提交信息
- `--json`：JSON 格式输出

示例输出：
```
[快照 ab12cd] 2025-06-17T15:30:00  新增=2 修改=1 删除=0
```

#### 查看状态
查看当前工作目录相对于上次快照的变化：
```bash
rustory status
```
示例输出：
```
已修改: src/lib.rs
已新增: tests/test_api.rs
已删除: docs/old.md
```

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
- `user.name`：用户名
- `user.email`：邮箱
- `editor`：默认编辑器
- `diff.tool`：差异比较工具

## 高级功能

### 压缩历史
压缩特定时间段内的历史记录，减小仓库大小：
```bash
rustory gc --older-than 30d
```

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

## 贡献指南
欢迎贡献代码！请遵循以下步骤：
1. Fork 本仓库。
2. 创建一个新的分支：
   ```bash
   git checkout -b feature-branch
   ```
3. 进行更改并提交。
4. 推送到您的 Fork 仓库：
   ```bash
   git push origin feature-branch
   ```
5. 创建 Pull Request。

### 代码规范
- 代码格式：使用 `rustfmt`
- 代码质量：通过 `clippy` 检查
- 测试覆盖：为新功能添加测试

## 性能优化建议
1. 对于大型项目，考虑调整 `.rustory/config.toml` 中的缓存设置
2. 定期运行 `rustory gc` 清理历史
3. 适当设置忽略规则，排除大型二进制文件或自动生成文件

## 版本计划
- **0.2.0**: 增强差异比较功能
- **0.3.0**: 添加分支支持
- **1.0.0**: API 稳定，完整文档

## 许可证
本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。