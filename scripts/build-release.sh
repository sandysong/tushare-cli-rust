#!/bin/bash
# 跨平台构建脚本

set -e

VERSION=$(cargo pkgid | cut -d# -f2 | cut -d: -f2)
echo "Building Tushare CLI v${VERSION}"

# 创建输出目录
mkdir -p dist

# 构建不同平台的二进制文件
echo "Building for macOS ARM64..."
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/tushare dist/tushare-macos-arm64

echo "Building for macOS x86_64..."
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/tushare dist/tushare-macos-x64

echo "Building for Linux x86_64..."
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/tushare dist/tushare-linux-x64

echo "Building for Windows x86_64..."
cargo build --release --target x86_64-pc-windows-msvc
cp target/x86_64-pc-windows-msvc/release/tushare.exe dist/tushare-windows-x64.exe

echo "All builds completed successfully!"
echo "Output files:"
ls -lh dist/
