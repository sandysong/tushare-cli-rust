#!/bin/bash
# Tushare CLI Rust - GitHub Release 构建脚本

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Tushare CLI Rust - Release Builder${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# 检查版本号
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d '"' -f 2)
echo -e "版本号: ${YELLOW}$VERSION${NC}"

# 检查是否已有 tag
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
    echo -e "${RED}错误: Tag v$VERSION 已存在${NC}"
    echo "请先更新 Cargo.toml 中的版本号"
    exit 1
fi

# 1. 构建 release
echo ""
echo -e "${GREEN}[1/4] 构建 Release...${NC}"
cargo build --release

# 检查构建结果
if [ ! -f "target/release/tushare" ]; then
    echo -e "${RED}错误: 构建失败${NC}"
    exit 1
fi

# 获取二进制文件大小
SIZE=$(du -h target/release/tushare | cut -f1)
echo -e "  ✓ 二进制大小: ${YELLOW}$SIZE${NC}"

# 2. 准备 skill 目录
echo ""
echo -e "${GREEN}[2/4] 准备 Skill 文件...${NC}"
mkdir -p skill/scripts
cp target/release/tushare skill/scripts/
echo -e "  ✓ CLI 二进制已复制"

# 3. 运行测试
echo ""
echo -e "${GREEN}[3/4] 运行测试...${NC}"
cargo test --quiet
echo -e "  ✓ 测试通过"

# 4. 创建 Git tag
echo ""
echo -e "${GREEN}[4/4] 创建 Git Tag...${NC}"
git tag -a "v$VERSION" -m "Release v$VERSION

$(cat <<'EOF'
## 🚀 Tushare CLI v{VERSION}

### ✨ 新特性
- 支持 238 个 Tushare Pro API 接口
- 零依赖单一可执行文件
- 多种输出格式（JSON、Table、CSV、Markdown）
- 完整的 Claude Code skill 集成

### 📦 下载
- **Linux/macOS**: tushare-v{VERSION}-x86_64-apple-darwin.tar.gz
EOF
)"
echo -e "  ✓ Tag v$VERSION 已创建"

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  构建完成！${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "下一步操作："
echo ""
echo -e "1. 推送到 GitHub："
echo -e "   ${YELLOW}git push origin main${NC}"
echo -e "   ${YELLOW}git push origin v$VERSION${NC}"
echo ""
echo -e "2. 创建 GitHub Release："
echo -e "   访问: ${YELLOW}https://github.com/sandysong/tushare-cli-rust/releases/new${NC}"
echo -e "   Tag: ${YELLOW}v$VERSION${NC}"
echo -e "   标题: ${YELLOW}v$VERSION${NC}"
echo ""
echo -e "3. 上传构建产物："
echo -e "   ${YELLOW}./scripts/upload-release.sh v$VERSION${NC}"
echo ""
