#!/bin/bash

# Tushare CLI Rust Skill 安装脚本
# 支持一键安装到 Claude Code Skills 目录

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 版本号
VERSION="${VERSION:-v1.0.2}"
REPO_URL="https://github.com/sandysong/tushare-cli-rust"
RELEASE_URL="${REPO_URL}/releases/download/${VERSION}"

# Claude Code skills 目录
SKILLS_DIR="${HOME}/.claude/skills"
SKILL_NAME="tushare-cli"

# 打印函数
print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_step() {
    echo -e "\n${BLUE}▶${NC} $1"
}

# 检测操作系统和架构
detect_platform() {
    print_step "检测系统信息..."

    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Darwin)
            PLATFORM="darwin"
            ;;
        Linux)
            PLATFORM="linux"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            PLATFORM="win32"
            ;;
        *)
            print_error "不支持的操作系统: $OS"
            exit 1
            ;;
    esac

    case "$ARCH" in
        x86_64|amd64)
            ARCH="x64"
            ;;
        arm64|aarch64)
            ARCH="arm64"
            ;;
        *)
            print_error "不支持的架构: $ARCH"
            exit 1
            ;;
    esac

    PLATFORM_STR="${PLATFORM}-${ARCH}"
    print_success "检测到平台: $PLATFORM_STR"
}

# 检查依赖
check_dependencies() {
    print_step "检查依赖..."

    local missing=()

    # 检查 unzip
    if ! command -v unzip &> /dev/null; then
        missing+=("unzip")
    fi

    # 检查 curl 或 wget
    if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
        missing+=("curl 或 wget")
    fi

    if [ ${#missing[@]} -ne 0 ]; then
        print_error "缺少依赖: ${missing[*]}"
        echo ""
        echo "安装方法:"
        echo "  macOS: brew install ${missing[*]}"
        echo "  Ubuntu/Debian: sudo apt-get install ${missing[*]}"
        echo "  CentOS/RHEL: sudo yum install ${missing[*]}"
        exit 1
    fi

    print_success "依赖检查通过"
}

# 下载文件
download_file() {
    local url="$1"
    local output="$2"

    if command -v curl &> /dev/null; then
        curl -fsSL "$url" -o "$output"
    elif command -v wget &> /dev/null; then
        wget -q "$url" -O "$output"
    fi
}

# 安装 skill
install_skill() {
    print_step "下载 Tushare CLI Rust Skill ${VERSION}..."

    # 创建临时目录
    TEMP_DIR="$(mktemp -d)"
    trap "rm -rf $TEMP_DIR" EXIT

    # 确定 skill 文件名
    if [ "$PLATFORM" = "win32" ]; then
        SKILL_FILE="tushare-cli-rust-win32-x64.skill"
    else
        SKILL_FILE="tushare-cli-rust-${PLATFORM_STR}.skill"
    fi

    DOWNLOAD_URL="${RELEASE_URL}/${SKILL_FILE}"

    print_info "下载地址: $DOWNLOAD_URL"

    # 下载 skill 文件
    SKILL_PATH="${TEMP_DIR}/${SKILL_FILE}"
    if ! download_file "$DOWNLOAD_URL" "$SKILL_PATH"; then
        print_error "下载失败"
        print_info "请检查版本号是否正确，或访问 $REPO_URL/releases 查看可用版本"
        exit 1
    fi

    print_success "下载完成"

    # 解压安装
    print_step "安装到 Claude Code Skills 目录..."

    # 创建 skills 目录（如果不存在）
    mkdir -p "$SKILLS_DIR"

    # 删除旧版本（如果存在）
    if [ -d "${SKILLS_DIR}/${SKILL_NAME}" ]; then
        print_warning "发现旧版本，正在删除..."
        rm -rf "${SKILLS_DIR}/${SKILL_NAME}"
    fi

    # 解压 skill 文件
    if ! unzip -o -q "$SKILL_PATH" -d "${SKILLS_DIR}/${SKILL_NAME}/"; then
        print_error "解压失败"
        exit 1
    fi

    print_success "安装完成"
}

# 设置权限
setup_permissions() {
    print_step "设置执行权限..."

    EXECUTABLE="${SKILLS_DIR}/${SKILL_NAME}/scripts/tushare"
    if [ "$PLATFORM" = "win32" ]; then
        EXECUTABLE="${SKILLS_DIR}/${SKILL_NAME}/scripts/tushare.exe"
    fi

    if [ -f "$EXECUTABLE" ]; then
        chmod +x "$EXECUTABLE" 2>/dev/null || true
        print_success "权限设置完成"
    else
        print_warning "未找到可执行文件，跳过权限设置"
    fi
}

# 验证安装
verify_installation() {
    print_step "验证安装..."

    if [ "$PLATFORM" = "win32" ]; then
        EXECUTABLE="${SKILLS_DIR}/${SKILL_NAME}/scripts/tushare.exe"
    else
        EXECUTABLE="${SKILLS_DIR}/${SKILL_NAME}/scripts/tushare"
    fi

    if [ ! -f "$EXECUTABLE" ]; then
        print_error "安装验证失败：可执行文件不存在"
        exit 1
    fi

    if "$EXECUTABLE" --version &> /dev/null; then
        VERSION_INFO=$("$EXECUTABLE" --version 2>&1 || echo "Unknown")
        print_success "版本: $VERSION_INFO"
    else
        print_warning "无法获取版本信息，但文件已存在"
    fi
}

# 配置提示
show_config_help() {
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}          安装成功！🎉${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "安装位置: ${BLUE}${SKILLS_DIR}/${SKILL_NAME}/${NC}"
    if [ "$PLATFORM" = "win32" ]; then
        echo -e "可执行文件: ${BLUE}${SKILLS_DIR}/${SKILL_NAME}/scripts/tushare.exe${NC}"
    else
        echo -e "可执行文件: ${BLUE}${SKILLS_DIR}/${SKILL_NAME}/scripts/tushare${NC}"
    fi
    echo ""
    echo -e "${YELLOW}下一步：配置 Tushare Token${NC}"
    echo ""
    echo "1. 获取 Token:"
    echo "   访问 https://tushare.pro 注册账号并获取 Token"
    echo ""
    echo "2. 配置环境变量:"
    echo "   ${BLUE}export TUSHARE_TOKEN=\"your_token_here\"${NC}"
    echo ""
    echo "   永久配置（添加到 shell 配置文件）："
    if [ "$PLATFORM" = "darwin" ]; then
        echo "   ${BLUE}echo 'export TUSHARE_TOKEN=\"your_token\"' >> ~/.zshrc${NC}"
        echo "   ${BLUE}source ~/.zshrc${NC}"
    elif [ "$PLATFORM" = "linux" ]; then
        echo "   ${BLUE}echo 'export TUSHARE_TOKEN=\"your_token\"' >> ~/.bashrc${NC}"
        echo "   ${BLUE}source ~/.bashrc${NC}"
    fi
    echo ""
    echo "3. 测试安装:"
    if [ "$PLATFORM" = "win32" ]; then
        echo "   ${BLUE}${SKILLS_DIR}/${SKILL_NAME}/scripts/tushare.exe --version${NC}"
    else
        echo "   ${BLUE}${SKILLS_DIR}/${SKILL_NAME}/scripts/tushare --version${NC}"
    fi
    echo ""
    echo "4. 在 Claude Code 中使用:"
    echo "   直接对 Claude 说：\"帮我查询平安银行的基本信息\""
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# 卸载函数
uninstall() {
    print_step "卸载 Tushare CLI Rust Skill..."

    if [ -d "${SKILLS_DIR}/${SKILL_NAME}" ]; then
        rm -rf "${SKILLS_DIR}/${SKILL_NAME}"
        print_success "卸载完成"
    else
        print_warning "未找到已安装的 Tushare CLI Rust Skill"
    fi
}

# 更新函数
update() {
    print_info "更新到最新版本..."

    # 获取最新版本号
    LATEST_VERSION=$(curl -fsSL "${REPO_URL}/releases/latest" | grep -o 'tag/[^"]*' | sed 's/tag\///' | head -1)

    if [ -z "$LATEST_VERSION" ]; then
        print_error "无法获取最新版本信息"
        exit 1
    fi

    print_info "最新版本: $LATEST_VERSION"

    # 设置版本号并安装
    VERSION="$LATEST_VERSION" install_skill
}

# 显示帮助
show_help() {
    cat << EOF
Tushare CLI Rust Skill 安装脚本

用法:
  ./install.sh [命令]

命令:
  install     安装 Tushare CLI Rust Skill (默认)
  uninstall   卸载 Tushare CLI Rust Skill
  update      更新到最新版本
  help        显示此帮助信息

环境变量:
  VERSION     指定安装版本 (默认: v1.0.0)

示例:
  # 安装默认版本
  ./install.sh

  # 安装指定版本
  VERSION=v1.1.0 ./install.sh

  # 更新到最新版本
  ./install.sh update

  # 卸载
  ./install.sh uninstall

一键安装:
  curl -fsSL https://raw.githubusercontent.com/sandysong/tushare-cli-rust/main/install.sh | bash

更多信息:
  https://github.com/sandysong/tushare-cli-rust
EOF
}

# 主函数
main() {
    echo -e "${BLUE}"
    cat << "EOF"
╔═══════════════════════════════════════════════════════════╗
║   Tushare CLI Rust for Claude Code - 安装向导             ║
╚═══════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"

    # 解析命令
    COMMAND="${1:-install}"

    case "$COMMAND" in
        install)
            check_dependencies
            detect_platform
            install_skill
            setup_permissions
            verify_installation
            show_config_help
            ;;
        uninstall)
            uninstall
            ;;
        update)
            check_dependencies
            detect_platform
            update
            setup_permissions
            verify_installation
            show_config_help
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "未知命令: $COMMAND"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# 运行主函数
main "$@"
