# Tushare CLI Rust - Claude Code Skill

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tushare](https://img.shields.io/badge/Tushare-Pro-blue)](https://tushare.pro)
[![Claude Code](https://img.shields.io/badge/Claude_Code-Skill-purple)](https://claude.ai/code)
[![Rust](https://img.shields.io/badge/Rust-1.83-orange.svg)](https://www.rust-lang.org)

> 🚀 用 Rust 编写的高性能 Tushare Pro CLI，支持 238 个 API 接口

## ✨ 功能特点

- ✅ **238 个 API 接口** - 支持所有 Tushare Pro 数据接口
- ✅ **零依赖部署** - 单一可执行文件，无需 Python SDK
- ✅ **极致性能** - 启动时间 ~20ms，内存占用 ~5MB
- ✅ **超小体积** - 编译后仅 2.3MB（相比 Bun 版本减少 95%）
- ✅ **多种输出格式** - JSON、Table、CSV、Markdown
- ✅ **自动参数转换** - 支持 kebab-case 到 snake_case 自动转换
- ✅ **智能搜索** - 快速搜索和查找接口

## 📦 安装

### 一键安装（推荐）

#### macOS / Linux

使用 bash 脚本自动完成安装：

```bash
curl -fsSL https://raw.githubusercontent.com/sandysong/tushare-cli-rust/main/install.sh | bash
```

#### Windows

使用 PowerShell 脚本自动完成安装：

```powershell
# 在 PowerShell 中运行
iex (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/sandysong/tushare-cli-rust/main/install.ps1" -UseBasicParsing).Content
```

安装脚本会自动：
- 检测您的操作系统和架构
- 下载对应的 `.skill` 文件
- 解压到 Claude Code skills 目录（`~/.claude/skills/` 或 `%USERPROFILE%\.claude\skills\`）
- 配置执行权限
- 验证安装

### 手动安装

从 [GitHub Releases](https://github.com/sandysong/tushare-cli-rust/releases) 下载对应平台的 `.skill` 文件：

#### macOS / Linux

```bash
# 1. 下载适合您平台的 tushare-cli-rust-*.skill 文件
# macOS (Apple Silicon): tushare-cli-rust-darwin-arm64.skill
# macOS (Intel): tushare-cli-rust-darwin-x64.skill
# Linux: tushare-cli-rust-linux-x64.skill

# 2. 解压到 Claude Code skills 目录
cd ~/.claude/skills
mkdir -p tushare-cli
unzip /path/to/tushare-cli-rust-*.skill -d tushare-cli

# 3. 配置 Tushare Token
export TUSHARE_TOKEN="your_token_here"

# 4. 验证安装
~/.claude/skills/tushare-cli/scripts/tushare --version
```

#### Windows

```powershell
# 1. 下载 tushare-cli-rust-win32-x64.skill 文件

# 2. 解压到 Claude Code skills 目录
cd $env:USERPROFILE\.claude\skills
New-Item -ItemType Directory -Path "tushare-cli" -Force
Expand-Archive -Path "C:\path\to\tushare-cli-rust-win32-x64.skill" -DestinationPath "tushare-cli" -Force

# 3. 配置 Tushare Token（永久）
[System.Environment]::SetEnvironmentVariable('TUSHARE_TOKEN', 'your_token_here', 'User')

# 4. 验证安装
.\tushare-cli\scripts\tushare.exe --version
```

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/sandysong/tushare-cli-rust.git
cd tushare-cli-rust

# 编译 release 版本
cargo build --release

# 二进制文件位于 target/release/tushare
```

### 使用 Cargo 安装

```bash
cargo install tushare
```

## 🚀 快速开始

### 1. 获取 Tushare Token

1. 访问 https://tushare.pro 注册账号
2. 在"个人中心"获取 Token
3. 配置环境变量：
   ```bash
   export TUSHARE_TOKEN="your_token_here"
   ```

### 2. 在 Claude Code 中使用

```
用户: 帮我查询京东方A的最新股价
Claude: [自动调用 tushare skill]
~/.claude/skills/tushare-cli/scripts/tushare daily --ts-code 000725.SZ --format markdown
```

### 3. 命令行直接使用

```bash
# 获取股票列表
~/.claude/skills/tushare-cli/scripts/tushare stock_basic --list-status L --format markdown

# 获取日线行情
~/.claude/skills/tushare-cli/scripts/tushare daily \
  --ts-code 000001.SZ \
  --start-date 20240101 \
  --end-date 20240131 \
  --format markdown

# 搜索接口
~/.claude/skills/tushare-cli/scripts/tushare search 股票

# 查看接口帮助
~/.claude/skills/tushare-cli/scripts/tushare help daily
```

## 📊 支持的数据类型

| 类别 | 接口数量 | 说明 |
|------|---------|------|
| 股票数据 | 116 | A股行情、财务、交易、筹码、管理层等 |
| 指数数据 | 14 | 各类指数行情、权重、行业分类 |
| 基金数据 | 16 | 基金净值、持仓、分红、规模 |
| 期货数据 | 13 | 期货合约行情、仓单、持仓 |
| 债券数据 | 12 | 可转债、国债、企业债 |
| 期权数据 | 4 | 期权合约信息、行情 |
| 宏观经济 | 15 | GDP、CPI、PPI、利率等 |
| 港股数据 | 9 | 港股行情、财务数据 |
| 美股数据 | 9 | 美股行情、财务数据 |
| 其他 | 20 | 龙虎榜、分红送股、新闻、公告等 |

**总计**: 238 个接口

## 💡 输出格式

支持 4 种输出格式：

### JSON（默认）
```bash
tushare stock_basic --format json --pretty
```

### Markdown（推荐用于对话）
```bash
tushare stock_basic --format markdown
```

### CSV（适合导入 Excel）
```bash
tushare stock_basic --format csv > stocks.csv
```

### Table（终端友好）
```bash
tushare stock_basic --format table
```

## 🔍 查找接口

### 列出所有接口
```bash
tushare list
```

### 按类别查看
```bash
tushare list 股票数据
tushare list 基金数据
```

### 搜索接口
```bash
# 搜索股票相关接口
tushare search 股票

# 搜索财务相关接口
tushare search 财务
```

### 查看接口详情
```bash
tushare help daily
tushare help fina_indicator
```

## 📚 文档

- **API 索引**: [skill/references/api-index.md](skill/references/api-index.md)
- **Skill 文档**: [skill/SKILL.md](skill/SKILL.md)
- **Tushare 官方文档**: https://tushare.pro/document/2

## 📈 性能对比

| 指标 | Bun 版本 | Rust 版本 | 改进 |
|------|---------|----------|------|
| 二进制大小 | 58MB | 2.3MB | ↓ 95% |
| 启动时间 | ~200ms | ~20ms | ↓ 90% |
| 内存占用 | ~50MB | ~5MB | ↓ 90% |

## 🛠️ 开发

### 构建

```bash
# 构建 debug 版本
cargo build

# 构建 release 版本
cargo build --release
```

### 测试

```bash
# 运行测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture
```

### 代码检查

```bash
# 检查代码
cargo check

# 格式化代码
cargo fmt

# 运行 linter
cargo clippy
```

## 📂 项目结构

```
tushare-cli-rust/
├── skill/                      # Claude Code skill 文件
│   ├── SKILL.md               # Skill 主文档
│   ├── references/
│   │   └── api-index.md       # API 接口索引（238个）
│   └── scripts/
│       └── tushare            # CLI 二进制文件
├── src/
│   ├── main.rs                # 主入口
│   ├── client/                # HTTP 客户端
│   ├── cli/                   # CLI 参数解析
│   ├── output/                # 输出格式化
│   ├── api/                   # API 定义（238个接口）
│   ├── config/                # 配置管理
│   └── error.rs               # 错误类型
├── tests/                     # 测试文件
├── install.sh                 # 一键安装脚本（macOS/Linux）
├── install.ps1                # 一键安装脚本（Windows）
└── Cargo.toml                 # 项目配置
```

## ⚙️ 配置

### 环境变量

```bash
# 必需：Tushare Token
export TUSHARE_TOKEN="your_token_here"

# 可选：添加到 shell 配置文件永久生效
echo 'export TUSHARE_TOKEN="your_token"' >> ~/.zshrc  # zsh
echo 'export TUSHARE_TOKEN="your_token"' >> ~/.bash_profile  # bash
```

### 参数格式

- **日期**: YYYYMMDD（如 20240131）
- **股票代码**: ts_code 格式（如 000001.SZ, 600000.SH）
- **参数命名**: 支持 kebab-case（自动转换为 snake_case）
  - `--ts-code` → `ts_code`
  - `--start-date` → `start_date`

## 🔧 常见问题

### Q: Token 未配置错误
```bash
错误: TUSHARE_TOKEN 未设置
```

**解决方案**:
```bash
export TUSHARE_TOKEN="your_token_here"
```

### Q: 权限不足错误
```bash
错误: 抱歉，您还没有获得该接口的调取权限
```

**解决方案**:
- 某些接口需要更高积分
- 访问 https://tushare.pro 查看积分规则
- 完成任务获取积分，或升级到付费版本

### Q: macOS 安全提示

如果 macOS 提示"无法验证开发者"：
```bash
xattr -d com.apple.quarantine ~/.claude/skills/tushare-cli/scripts/tushare
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- [Tushare Pro](https://tushare.pro) - 提供金融数据 API
- [Rust](https://www.rust-lang.org) - 高性能系统编程语言
- [Claude Code](https://claude.ai/code) - AI 编程助手

---

**免责声明**: 本工具仅供学习和研究使用，请遵守 Tushare API 使用条款。投资有风险，数据仅供参考。
