## 概述

**rustory** 是一款用 Rust 完全实现的轻量级本地版本管理工具，支持 Linux、macOS 和 Windows 等多平台。无需依赖外部命令或脚本解释器，即可跟踪、快照和还原项目文件变更。

- **目标用户**：个人开发者、脚本作者，需要在本地维护文件历史，无需分布式协作。
- **存储位置**：项目根目录下的隐藏目录 `.rustory/`。
- **技术栈**：
  - Rust 标准库 + 社区成熟 crate：`clap`, `walkdir`, `sha1`/`sha2`, `serde`/`toml`/`serde_json`, `ignore`, `tar`+`flate2`/`zip`。

## 目录结构

```
~/my_project/
├── src/
├── README.md
└── .rustory/
    ├── config.toml           # 用户配置：忽略规则、输出格式、备份策略等
    ├── ignore                # 忽略规则文件（Git 样式）
    ├── objects/              # 按 SHA-1 哈希存储内容
    ├── index.json            # 当前工作区文件与哈希映射
    ├── history.log           # 快照日志：ID、时间、改动统计、备注
    └── snapshots/            # 快照元数据 JSON 文件
```

## 核心概念

1. **对象存储**：将文件内容写为二进制对象，文件名为其 SHA-1 哈希，实现去重。

2. **索引（index.json）**：记录工作区文件路径与对应哈希，用于检测变更。

3. **快照（snapshot）**：保存一次索引状态，元数据存于 `snapshots/`，并记录在 `history.log`。

4. **日志（history.log）**：每行一条，格式：

   ```
   <snapshot_id> <ISO8601 时间> <added>/<modified>/<deleted> msg="..."
   ```

## 子命令

### `rustory init`

初始化仓库：

1. 创建 `.rustory/` 目录和子结构。
2. 生成默认 `config.toml` 和 `.rustory/ignore`。
3. 扫描工作区，构建首个对象和快照，写入 `index.json` 和 `history.log`。

```
$ rustory init
Initialized empty rustory repository in .rustory/
```

### `rustory commit [-m <msg>] [--json]`  (别名：`rustory log`)

创建新快照：

- 扫描并比较 `index.json`，检测新增、修改、删除文件。
- 保存新对象，更新 `index.json`，追加 `history.log` 记录。
- `-m`: 添加备注； `--json`: 输出结构化结果。

```
$ rustory commit -m "Fix bug in parser"
[snapshot ab12cd] 2025-06-17T15:30:00  added=2 modified=1 deleted=0
```

### `rustory history` (别名：`rustory list`)

显示所有快照记录：

- 默认表格，列出 ID、时间、改动统计、备注。
- `--json`: 输出 JSON 数组。

```
$ rustory history
ID       Time                     +  ~  -  Message
ab12cd   2025-06-17T15:30:00      2  1  0  "Fix bug in parser"
```

### `rustory status`

展示工作区相对 `index.json` 的文件差异，并高亮新增、修改、删除列表。

```
$ rustory status
Modified: src/lib.rs
Added:    tests/test_api.rs
Deleted:  docs/old.md
```

### `rustory diff [<id1>] [<id2>]`

对比两个快照或快照与当前工作区：

- 无参数：与当前工作区比较。
- 一个 ID：与当前工作区比较；两个 ID：快照间对比。
- 支持行级、字符级差异。

```
$ rustory diff ab12cd ab34ef
diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -10,6 +10,7 @@ fn parse() {
```

### `rustory rollback <id> [--restore] [--keep-index]`

管理快照恢复：

- **默认**：将指定快照内容导出到 `backup-<timestamp>/`，**不** 改动当前工作区。
- `--restore`: 直接用快照替换当前工作区内容（并备份原始）；
- `--keep-index`: 不更新 `index.json`。

```
$ rustory rollback ab12cd
Exported snapshot ab12cd to backup-2025-06-17T16-00-00/
```

### `rustory tag <name> <id>`

为快照添加标签，方便引用，标签信息写入 `config.toml`。

```
$ rustory tag v1.0 ab12cd
Tagged snapshot ab12cd as "v1.0"
```

### `rustory ignore [edit|show]`

管理 `.rustory/ignore`：

- `show`: 显示当前规则；
- `edit`: 启动配置的编辑器修改规则，修改后自动生效。

### `rustory config [set|get] <key> [value]`

查看或修改 `config.toml` 中的配置项。

### `rustory help [command]`

显示全局或指定命令的帮助信息，基于 `clap` 自动生成。

## 交互与可扩展性

- **跨平台**：无需外部依赖，Rust 原生。
- **配置化**：`config.toml` 支持忽略规则、输出格式、备份策略等。
- **脚本友好**：所有命令均支持 `--json`。
- **插件**：未来可引入自定义差异算法、存储后端等。

## 错误处理

- **未初始化**：`fatal: not a rustory repository (or any parent up to root)`。
- **快照/标签不存在**：退出码 `1`，`error: snapshot '<id>' not found`。
- **参数错误**：退出码 `2`，显示用法帮助。
- **大文件警告**（>100MB）：提示并跳过，或 `--force` 强制处理。