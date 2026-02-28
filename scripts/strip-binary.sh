#!/bin/bash
# 二进制压缩脚本 (使用 UPX)

set -e

BINARY_PATH="${1:-target/release/tushare}"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

# 检查 UPX 是否安装
if ! command -v upx &> /dev/null; then
    echo "Warning: UPX not found. Please install UPX for binary compression."
    echo "brew install upx  # macOS"
    echo "apt install upx   # Linux"
    exit 1
fi

echo "Compressing binary with UPX..."
ORIGINAL_SIZE=$(stat -f%z "$BINARY_PATH" 2>/dev/null || stat -c%s "$BINARY_PATH" 2>/dev/null)

# 使用 UPX 压缩 (最佳压缩)
upx --best --lzma "$BINARY_PATH"

NEW_SIZE=$(stat -f%z "$BINARY_PATH" 2>/dev/null || stat -c%s "$BINARY_PATH" 2>/dev/null)
REDUCTION=$(echo "scale=2; (1 - $NEW_SIZE / $ORIGINAL_SIZE) * 100" | bc)

echo "Compression complete!"
echo "Original size: $ORIGINAL_SIZE bytes"
echo "New size: $NEW_SIZE bytes"
echo "Reduction: ${REDUCTION}%"
