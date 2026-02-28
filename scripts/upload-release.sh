#!/bin/bash
# 上传构建产物到 GitHub Release

set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "用法: $0 <version>"
    echo "示例: $0 v1.0.0"
    exit 1
fi

# 检查 gh 是否可用
if ! command -v gh &> /dev/null; then
    echo "错误: GitHub CLI (gh) 未安装"
    echo ""
    echo "请安装 GitHub CLI:"
    echo "  macOS:  brew install gh"
    echo "  Linux: https://github.com/cli/cli/blob/trunk/docs/install_linux.md"
    echo ""
    echo "或手动上传以下文件到 Release:"
    echo "  - target/release/tushare"
    exit 1
fi

# 检查认证
if ! gh auth status &> /dev/null; then
    echo "错误: GitHub CLI 未认证"
    echo "请运行: gh auth login"
    exit 1
fi

echo "准备上传 Release v$VERSION..."

# 创建临时目录
DIST_DIR="dist-$VERSION"
mkdir -p "$DIST_DIR"

# 复制二进制文件
cp target/release/tushare "$DIST_DIR/"

# 创建压缩包
echo "创建压缩包..."
cd "$DIST_DIR"
tar czf "../tushare-$VERSION-x86_64-apple-darwin.tar.gz" tushare
cd ..

# 获取文件大小
SIZE=$(du -h "tushare-$VERSION-x86_64-apple-darwin.tar.gz" | cut -f1)
echo "  ✓ 压缩包大小: $SIZE"

# 上传到 GitHub Release
echo ""
echo "上传到 GitHub Release..."
gh release upload "v$VERSION" \
    "tushare-$VERSION-x86_64-apple-darwin.tar.gz" \
    --censor

# 清理
rm -rf "$DIST_DIR"

echo ""
echo "✓ 上传完成！"
echo ""
echo "Release: https://github.com/sandysong/tushare-cli-rust/releases/tag/$VERSION"
