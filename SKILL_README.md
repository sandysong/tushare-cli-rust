# Tushare CLI - Claude Code Skill

本项目包含一个完整的 Tushare Pro CLI 工具，可以作为 Claude Code skill 使用。

## 📦 项目结构

```
tushare-cli-rust/
├── skill/                      # Claude Code skill 文件
│   ├── skill.md               # Skill 主文档
│   ├── references/
│   │   └── api-index.md       # API 接口索引（238个接口）
│   └── scripts/
│       └── tushare            # CLI 二进制文件
├── src/                       # Rust 源代码
├── install-skill.sh           # Skill 安装脚本
└── Cargo.toml                 # 项目配置
```

## 🚀 安装为 Claude Code Skill

### 方法 1：使用安装脚本（推荐）

```bash
cd /Users/songqi/Work/quant/tushare-cli-rust
./install-skill.sh
```

### 方法 2：手动安装

```bash
# 1. 构建 CLI
cargo build --release

# 2. 创建 skill 目录
mkdir -p ~/.claude/skills/tushare-cli/{scripts,references}

# 3. 复制文件
cp skill/skill.md ~/.claude/skills/tushare-cli/
cp skill/references/api-index.md ~/.claude/skills/tushare-cli/references/
cp target/release/tushare ~/.claude/skills/tushare-cli/scripts/
```

## 📊 支持的 API

支持 **238 个 Tushare Pro API 接口**，包括：

| 类别 | 接口数量 |
|------|---------|
| 股票数据 | 108 |
| 宏观经济 | 21 |
| 指数专题 | 19 |
| 债券专题 | 15 |
| 期货数据 | 12 |
| 港股数据 | 11 |
| 美股数据 | 9 |
| ETF专题 | 8 |
| 公募基金 | 8 |
| 行业经济 | 8 |
| 大模型语料专题数据 | 6 |
| 期权数据 | 3 |
| 其他 | 7 |

完整 API 列表请查看 [skill/references/api-index.md](skill/references/api-index.md)

## 💡 使用示例

安装后，在 Claude Code 中可以直接使用：

```
你：帮我获取平安银行的基本信息
Claude：[自动调用 tushare stock_basic 接口]

你：查询 000001.SZ 最近一个月的日线行情
Claude：[自动调用 tushare daily 接口]
```

## 🛠️ 开发

### 构建

```bash
cargo build --release
```

### 测试

```bash
cargo test
```

### 运行

```bash
./target/release/tushare list
./target/release/tushare stock_basic --list-status L --format markdown
```

## 📝 许可证

MIT License

## 🔗 相关链接

- [Tushare 官方文档](https://tushare.pro/document/2)
- [API 测试工具](https://tushare.pro/document/1)
- [项目地址](https://github.com/sandysong/tushare-skill)
