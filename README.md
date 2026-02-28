# Tushare CLI - Rust 版本

获取中国金融市场数据的命令行工具（Rust 实现）

## 特点

- 🚀 **高性能** - 用 Rust 编写，启动快速，内存占用小
- 📦 **体积小** - 编译后的二进制文件仅 2-5MB（相比 Bun 版本的 60MB）
- 🔧 **零依赖部署** - 单一可执行文件，无需额外依赖
- 📊 **多格式输出** - 支持 JSON、Table、CSV、Markdown 四种输出格式
- 🎯 **完整 API 支持** - 支持 211+ Tushare Pro API 接口
- 🔍 **智能搜索** - 快速查找和搜索 API 接口

## 安装

### 使用预编译二进制

从 [Releases](https://github.com/sandysong/tushare-skill/releases) 下载对应平台的二进制文件：

```bash
# macOS (ARM64)
wget https://github.com/sandysong/tushare-skill/releases/latest/download/tushare-macos-arm64 -O tushare
chmod +x tushare

# Linux (x64)
wget https://github.com/sandysong/tushare-skill/releases/latest/download/tushare-linux-x64 -O tushare
chmod +x tushare
```

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/sandysong/tushare-skill.git
cd tushare-cli-rust

# 编译 release 版本
cargo build --release

# 二进制文件位于 target/release/tushare
```

### 使用 Cargo 安装

```bash
cargo install tushare
```

## 配置

设置 API Token（推荐）：

```bash
export TUSHARE_TOKEN="your_token_here"
```

或在运行时使用 `--token` 参数：

```bash
tushare --token "your_token_here" stock_basic
```

## 使用方法

### 基本用法

```bash
# 查看帮助
tushare --help
tushare help stock_basic

# 调用 API
tushare stock_basic --ts-code 000001.SZ

# 指定输出格式
tushare stock_basic --format json --pretty

# 获取日线行情
tushare daily --ts-code 000001.SZ --start-date 20240101
```

### 支持的命令

```bash
# 列出所有 API 接口
tushare list

# 按类别列出
tushare list 股票数据

# 搜索 API 接口
tushare search 龙虎榜
```

### 输出格式

```bash
# JSON 格式（美化）
tushare stock_basic --format json --pretty --ts-code 000001.SZ

# 表格格式（默认）
tushare stock_basic --ts-code 000001.SZ

# CSV 格式
tushare stock_basic --format csv --ts-code 000001.SZ

# Markdown 格式
tushare stock_basic --format markdown --ts-code 000001.SZ
```

### 参数格式

支持多种参数格式：

```bash
# 标准格式
tushare daily --ts-code 000001.SZ --start-date 20240101

# 等号格式
tushare daily --ts-code=000001.SZ --start-date=20240101

# kebab-case 自动转换为 snake_case
tushare daily --ts-code 000001.SZ  # 自动转换为 ts_code
```

## 性能对比

| 指标 | Bun 版本 | Rust 版本 | 改进 |
|------|---------|----------|------|
| 二进制大小 | 58MB | 2-3MB | ↓ 95% |
| 启动时间 | ~200ms | ~20ms | ↓ 90% |
| 内存占用 | ~50MB | ~5MB | ↓ 90% |

## 开发

```bash
# 运行测试
cargo test

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 运行 linter
cargo clippy

# 构建 release 版本
cargo build --release

# 生成 API 定义
./scripts/generate-definitions.sh
```

## 项目结构

```
tushare-cli-rust/
├── src/
│   ├── main.rs          # 主入口
│   ├── client/          # HTTP 客户端
│   ├── cli/             # CLI 参数解析
│   ├── output/          # 输出格式化
│   ├── api/             # API 定义
│   ├── config/          # 配置管理
│   └── error.rs         # 错误类型
├── scripts/             # 构建脚本
├── tests/               # 测试文件
└── Cargo.toml           # 项目配置
```

## 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 致谢

- [Tushare Pro](https://tushare.pro) - 提供金融数据 API
- 原版 [tushare-cli](https://github.com/sandysong/tushare-skill) (TypeScript/Bun 实现)

## 相关链接

- [Tushare 官方文档](https://tushare.pro/document/2)
- [API 文档](https://tushare.pro/document/1)
- [问题反馈](https://github.com/sandysong/tushare-skill/issues)
