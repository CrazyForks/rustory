# 发布前检查清单

在创建新的发布版本之前，请确保完成以下检查：

## 🔍 代码质量检查

- [ ] 所有测试通过
  ```bash
  cargo test
  ```

- [ ] 代码格式化正确
  ```bash
  cargo fmt --all -- --check
  ```

- [ ] 没有 Clippy 警告
  ```bash
  cargo clippy -- -D warnings
  ```

- [ ] 文档构建成功
  ```bash
  cargo doc --no-deps
  ```

## 📝 版本信息更新

- [ ] 更新 `Cargo.toml` 中的版本号
- [ ] 更新 `README.md` 中的版本信息（如果有）
- [ ] 添加 `CHANGELOG.md` 条目（如果有维护）
- [ ] 确认所有依赖项版本合适

## 🧪 本地构建测试

- [ ] 运行本地构建脚本
  ```bash
  ./scripts/build-local.sh
  ```

- [ ] 检查主要平台的构建结果
  - [ ] Linux x64
  - [ ] Windows x64 (如果可能)
  - [ ] macOS x64 (如果可能)

## 📋 发布准备

- [ ] 创建发布分支（如果使用 Git Flow）
  ```bash
  git checkout -b release/v1.0.0
  ```

- [ ] 提交所有更改
  ```bash
  git add .
  git commit -m "Prepare for release v1.0.0"
  ```

- [ ] 推送到远程仓库
  ```bash
  git push origin main
  # 或者推送发布分支
  git push origin release/v1.0.0
  ```

## 🏷️ 标签创建

- [ ] 创建带注释的 Git 标签
  ```bash
  git tag -a v1.0.0 -m "Release v1.0.0

  Major changes:
  - Feature A
  - Feature B
  - Bug fix C
  "
  ```

- [ ] 推送标签触发发布流程
  ```bash
  git push origin v1.0.0
  ```

## 🔄 发布后验证

- [ ] 监控 GitHub Actions 构建状态
- [ ] 验证所有平台的二进制文件都已生成
- [ ] 检查发布页面的文件和 SHA256 值
- [ ] 下载并测试至少一个平台的二进制文件
- [ ] 确认发布说明正确显示

## 🚨 紧急修复流程

如果发布后发现严重问题：

1. **立即修复**
   ```bash
   git checkout main
   git pull origin main
   # 修复问题
   git add .
   git commit -m "Hotfix: Fix critical issue"
   ```

2. **创建修复版本**
   ```bash
   # 增加 PATCH 版本号
   git tag -a v1.0.1 -m "Hotfix v1.0.1"
   git push origin v1.0.1
   ```

3. **撤回有问题的发布**
   - 在 GitHub 上将有问题的发布标记为 "Pre-release"
   - 或者删除发布（如果刚刚发布且无人下载）

## 📞 联系信息

如果在发布过程中遇到问题：
- 检查 GitHub Actions 日志
- 查看 Issues 页面是否有相关问题
- 联系项目维护者

---

**注意**: 这个检查清单应该根据项目的具体需求进行调整。
