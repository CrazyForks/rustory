#!/bin/bash

# 本地测试构建脚本
# 用于在发布前测试多平台构建

set -e

echo "🚀 开始本地多平台构建测试..."

# 定义目标平台
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
    "aarch64-pc-windows-msvc"
)

# 创建输出目录
OUTPUT_DIR="local-builds"
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

echo "📦 安装必要的目标平台..."
for target in "${TARGETS[@]}"; do
    echo "添加目标: $target"
    rustup target add "$target" || echo "⚠️  目标 $target 可能已存在或不可用"
done

echo ""
echo "🔨 开始构建各平台版本..."

for target in "${TARGETS[@]}"; do
    echo "构建目标: $target"
    
    # 尝试构建
    if cargo build --release --target "$target"; then
        echo "✅ $target 构建成功"
        
        # 确定可执行文件名
        if [[ "$target" == *"windows"* ]]; then
            exe_name="rustory.exe"
            archive_name="rustory-$target.zip"
        else
            exe_name="rustory"
            archive_name="rustory-$target.tar.gz"
        fi
        
        # 检查文件是否存在
        binary_path="target/$target/release/$exe_name"
        if [[ -f "$binary_path" ]]; then
            # 创建压缩包
            cd "target/$target/release"
            if [[ "$target" == *"windows"* ]]; then
                # Windows: 创建 ZIP (需要 zip 命令)
                if command -v zip >/dev/null 2>&1; then
                    zip "../../../$OUTPUT_DIR/$archive_name" "$exe_name"
                else
                    echo "⚠️  zip 命令不可用，跳过压缩包创建"
                    cp "$exe_name" "../../../$OUTPUT_DIR/"
                fi
            else
                # Unix: 创建 tar.gz
                tar -czf "../../../$OUTPUT_DIR/$archive_name" "$exe_name"
            fi
            cd - > /dev/null
            
            # 生成 SHA256
            if [[ -f "$OUTPUT_DIR/$archive_name" ]]; then
                if command -v shasum >/dev/null 2>&1; then
                    shasum -a 256 "$OUTPUT_DIR/$archive_name" | cut -d ' ' -f 1 > "$OUTPUT_DIR/$archive_name.sha256"
                elif command -v sha256sum >/dev/null 2>&1; then
                    sha256sum "$OUTPUT_DIR/$archive_name" | cut -d ' ' -f 1 > "$OUTPUT_DIR/$archive_name.sha256"
                else
                    echo "⚠️  SHA256 工具不可用"
                fi
            fi
        else
            echo "⚠️  二进制文件未找到: $binary_path"
        fi
    else
        echo "❌ $target 构建失败"
    fi
    echo ""
done

echo "📊 构建结果汇总:"
echo "===================="
ls -la "$OUTPUT_DIR/" 2>/dev/null || echo "输出目录为空"

echo ""
echo "🎉 本地构建测试完成！"
echo "💡 提示: 某些平台可能需要特殊的交叉编译工具才能成功构建"
echo "💡 在 CI/CD 环境中，这些工具会自动安装"
