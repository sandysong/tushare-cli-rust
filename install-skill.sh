#!/bin/bash
# 安装 tushare-cli-rust 为 Claude Code skill

SKILL_DIR="$HOME/.claude/skills/tushare-cli"

echo "正在安装 tushare-cli-rust skill..."

# 创建 skill 目录
mkdir -p "$SKILL_DIR/references"
mkdir -p "$SKILL_DIR/scripts"

# 复制 skill 文件
cp skill/skill.md "$SKILL_DIR/"
cp skill/references/api-index.md "$SKILL_DIR/references/"

# 构建 CLI（如果尚未构建）
if [ ! -f "target/release/tushare" ]; then
    echo "正在构建 CLI..."
    cargo build --release
fi

# 复制 CLI 二进制文件
cp target/release/tushare "$SKILL_DIR/scripts/"

echo "✓ 安装完成！"
echo ""
echo "Skill 已安装到: $SKILL_DIR"
echo ""
echo "使用方法："
echo "  1. 在 Claude Code 中可以直接使用 tushare 相关命令"
echo "  2. 例如: '帮我获取平安银行的基本信息'"
echo "  3. 或者: '查询 000001.SZ 的最近日线行情'"
