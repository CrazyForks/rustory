# 发布流程说明

本项目使用 GitHub Actions 自动构建和发布多平台二进制文件。

## 🚀 自动发布流程

### 触发条件
发布流程在以下情况下自动触发：
1. **推送标签**: 当推送以 `v` 开头的标签时（如 `v1.0.0`）
2. **手动触发**: 在 GitHub Actions 页面手动运行工作流

### 支持的平台

| 平台 | 架构 | 文件格式 | 目标三元组 |
|------|------|----------|------------|
| Windows | x64 | ZIP | `x86_64-pc-windows-msvc` |
| Windows | ARM64 | ZIP | `aarch64-pc-windows-msvc` |
| macOS | x64 (Intel) | tar.gz | `x86_64-apple-darwin` |
| macOS | ARM64 (Apple Silicon) | tar.gz | `aarch64-apple-darwin` |
| Linux | x64 | tar.gz | `x86_64-unknown-linux-gnu` |
| Linux | ARM64 | tar.gz | `aarch64-unknown-linux-gnu` |

## 📋 发布新版本

### 1. 准备发布
```bash
# 确保所有更改已提交
git add .
git commit -m "Prepare for release v1.0.0"

# 更新版本号 (可选，在 Cargo.toml 中)
# version = "1.0.0"

# 推送更改
git push origin main
```

### 2. 创建发布标签
```bash
# 创建带注释的标签
git tag -a v1.0.0 -m "Release v1.0.0"

# 推送标签以触发发布流程
git push origin v1.0.0
```

### 3. 监控构建过程
1. 访问 GitHub repository 的 Actions 页面
2. 查看 "Release" 工作流的运行状态
3. 构建完成后，检查 Releases 页面的新发布

## 🔍 发布内容

每个发布版本包含：

### 二进制文件
- 6个平台的预编译二进制文件
- 压缩格式：Windows 使用 ZIP，其他平台使用 tar.gz

### 校验文件
- 每个二进制文件对应的 SHA256 校验文件
- 文件名格式：`rustory-{target}.{ext}.sha256`

### 发布说明
自动生成的发布说明包含：
- 按平台分组的下载链接
- 每个文件的 SHA256 值
- 校验方法说明

## 🔧 工作流配置

### 主要文件
- `.github/workflows/release.yml`: 发布工作流
- `.github/workflows/ci.yml`: 持续集成工作流
- `.cargo/config.toml`: Cargo 构建配置

### 构建优化
发布版本使用以下优化设置：
- `opt-level = 3`: 最高优化级别
- `lto = true`: 链接时优化
- `codegen-units = 1`: 单个代码生成单元
- `strip = true`: 移除调试符号
- `panic = "abort"`: 使用 abort 策略

## 🛠️ 故障排除

### 常见问题

**1. 交叉编译失败**
- Linux ARM64: 确保安装了 `gcc-aarch64-linux-gnu`
- Windows ARM64: 依赖 Rust 工具链支持
- macOS ARM64: 需要 Xcode 命令行工具

**2. 构建超时**
- 检查依赖项是否正确缓存
- 考虑减少并行构建数量

**3. 发布失败**
- 确保有推送标签的权限
- 检查 GITHUB_TOKEN 权限设置

### 手动发布
如果自动发布失败，可以手动运行：
```bash
# 本地构建所有目标
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
# ... 其他目标

# 创建压缩包
cd target/x86_64-unknown-linux-gnu/release
tar -czf rustory-x86_64-unknown-linux-gnu.tar.gz rustory

# 生成校验值
shasum -a 256 rustory-x86_64-unknown-linux-gnu.tar.gz > rustory-x86_64-unknown-linux-gnu.tar.gz.sha256
```

## 📝 版本号管理

建议遵循 [语义化版本](https://semver.org/lang/zh-CN/) 规则：
- `MAJOR.MINOR.PATCH`
- 主版本号：不兼容的 API 修改
- 次版本号：向下兼容的功能性新增
- 修订号：向下兼容的问题修正

示例标签：
- `v1.0.0`: 首个稳定版本
- `v1.1.0`: 新功能发布
- `v1.1.1`: 问题修复
- `v2.0.0`: 重大更新
