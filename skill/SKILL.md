---
name: tushare-cli-rust
description: 使用 Tushare CLI 工具获取中国金融市场数据。支持 238 个 Tushare Pro 数据接口，包括股票行情、财务数据、宏观经济指标等。使用 Rust 编写的零依赖命令行工具直接调用 HTTP API，无需 Python SDK。务必在用户涉及中国股市、A股、港股、美股、基金、经济指标、财务报表、股票行情、K线数据、日线/周线/月线、技术分析、财务指标、上市公司数据、指数数据、期货、债券等任何中国金融市场数据查询需求时使用此 skill，即使用户没有明确提及 Tushare 或 CLI。
---

# Tushare CLI Skill

使用 Tushare CLI 工具获取中国金融市场数据的技能。

## CLI 工具路径

```
scripts/tushare
```

## 📋 数据覆盖

支持 **238 个 Tushare Pro API 接口**，包括：

- **股票数据**（108个）：基础信息、行情、财务、交易、筹码、管理层等
- **宏观经济**（21个）：GDP、CPI、PPI、利率、货币供应等
- **指数专题**（19个）：基础信息、日线、权重、行业指数
- **债券专题**（15个）：可转债、国债、企业债、回购等
- **期货数据**（12个）：合约、行情、仓单、持仓等
- **港股数据**（11个）：行情、财务数据
- **美股数据**（9个）：行情、财务数据
- **ETF专题**（8个）：ETF基本信息、行情、份额等
- **公募基金**（8个）：基金净值、持仓、分红、规模
- **行业经济**（8个）：TMT行业、影视票房等
- **大模型语料专题数据**（6个）：新闻、公告、研报、政策等
- **期权数据**（3个）：合约信息、行情
- **财富管理**（2个）：基金销售数据
- **外汇数据**（2个）：外汇基础信息、行情
- **现货数据**（2个）：黄金现货数据
- **基金数据**（1个）：基金相关数据
- **其他**（3个）：其他数据接口

> 💡 **完整的接口列表**：参见 [references/api-index.md](references/api-index.md)

## 功能特点

- **零依赖**：单一可执行文件，无需 Python 或其他运行时
- **多种输出格式**：JSON、Table、CSV、Markdown
- **自动参数转换**：支持 kebab-case 到 snake_case 的自动转换
- **智能搜索**：支持按关键词搜索接口

## 命令格式

### 基本格式

```bash
scripts/tushare <接口名> [参数] [选项]
```

### 通用选项

- `-h, --help`: 显示帮助信息
- `-v, --version`: 显示版本信息
- `-f, --format`: 输出格式 (json|table|csv|markdown)
- `-p, --pretty`: JSON 美化输出
- `-t, --token`: Tushare API Token

### 参数格式

- **日期**：YYYYMMDD（如 20240228）
- **股票代码**：ts_code 格式（如 000001.SZ, 600000.SH）
- **参数命名**：支持 kebab-case（自动转换为 snake_case）
  - `--ts-code` → `ts_code`
  - `--start-date` → `start_date`

### 输出格式选择

| 格式 | 适用场景 | 示例命令 |
|------|---------|---------|
| **markdown** | 在对话中展示数据（推荐） | `--format markdown` |
| **csv** | 需要进一步处理分析 | `--format csv` |
| **json** | 程序处理 | `--format json` |
| **table** | 终端直接查看 | `--format table` |

## 常用接口速查

| 数据类型 | 接口方法 | 说明 |
|---------|---------|------|
| 股票列表 | `stock_basic` | 获取所有股票列表 |
| 日线行情 | `daily` | 获取日线行情数据 |
| 周线行情 | `weekly` | 获取周线行情数据 |
| 月线行情 | `monthly` | 获取月线行情数据 |
| 财务指标 | `fina_indicator` | 财务指标（ROE等） |
| 利润表 | `income` | 利润表数据 |
| 资产负债表 | `balancesheet` | 资产负债表数据 |
| 现金流量表 | `cashflow` | 现金流量表数据 |
| 指数行情 | `index_daily` | 指数日线数据 |
| 基金净值 | `fund_nav` | 基金净值数据 |
| GDP数据 | `cn_gdp` | 国内生产总值 |
| CPI数据 | `cn_cpi` | 居民消费价格指数 |

## 工作流程

当用户请求获取金融数据时，遵循以下流程：

### 1. 理解需求

- 用户需要什么类型的数据？（股票、基金、宏观经济等）
- 具体的查询参数？（股票代码、日期范围等）
- 数据用途？（直接展示、技术分析、对比研究）

### 2. 直接执行查询（不要预检查 Token）

**重要**：直接执行命令，不要预先询问或检查 Token 配置。

```bash
# 示例：直接执行查询
scripts/tushare daily \
  --ts-code 000725.SZ \
  --start-date 20240201 \
  --end-date 20250228 \
  --format csv
```

### 3. 处理结果

- **成功**：直接展示数据或进行分析
- **Token 错误**：提示用户配置 Token（见错误处理部分）
- **权限错误**：提示用户积分不足
- **参数错误**：检查参数格式并重试

### 4. 数据分析（如需要）

对于需要分析的场景（如技术指标分析）：
1. 使用 `--format csv` 获取数据
2. 使用命令行工具（如 `awk`、`csvtool`）进行分析
3. 或者直接展示数据并提供分析思路

**不要**：编写 Python 脚本或创建临时文件进行处理

## 使用示例

### 股票基础信息

```bash
# 获取股票基本信息
scripts/tushare stock_basic \
  --ts-code 000725.SZ \
  --format markdown
```

### 行情数据

```bash
# 获取日线数据（推荐用 csv 格式便于处理）
scripts/tushare daily \
  --ts-code 000725.SZ \
  --start-date 20240201 \
  --end-date 20250228 \
  --format csv

# 获取特定日期的全市场行情
scripts/tushare daily \
  --trade-date 20240228 \
  --format markdown

# 获取周线数据
scripts/tushare weekly \
  --ts-code 000725.SZ \
  --start-date 20230101 \
  --end-date 20231231 \
  --format csv

# 获取月线数据
scripts/tushare monthly \
  --ts-code 000725.SZ \
  --start-date 20220101 \
  --end-date 20231231 \
  --format csv
```

### 财务数据

```bash
# 获取利润表
scripts/tushare income \
  --ts-code 000725.SZ \
  --start-date 20230101 \
  --end-date 20231231 \
  --format markdown

# 获取资产负债表
scripts/tushare balancesheet \
  --ts-code 000725.SZ \
  --period 20231231 \
  --format markdown

# 获取现金流量表
scripts/tushare cashflow \
  --ts-code 000725.SZ \
  --period 20231231 \
  --format markdown

# 获取财务指标
scripts/tushare fina_indicator \
  --ts-code 000725.SZ \
  --start-date 20230101 \
  --end-date 20231231 \
  --format markdown
```

### 指数数据

```bash
# 获取指数基本信息
scripts/tushare index_basic \
  --market SSE \
  --format markdown

# 获取指数日线数据
scripts/tushare index_daily \
  --ts-code 000001.SH \
  --start-date 20240201 \
  --format csv
```

### 基金数据

```bash
# 获取基金列表
scripts/tushare fund_basic \
  --market E \
  --format markdown

# 获取基金净值
scripts/tushare fund_nav \
  --ts-code 165509.SZ \
  --start-date 20240201 \
  --format csv
```

### 宏观经济数据

```bash
# GDP 数据
scripts/tushare cn_gdp --format markdown

# CPI 数据
scripts/tushare cn_cpi --format markdown

# PPI 数据
scripts/tushare cn_ppi --format markdown

# Shibor 利率
scripts/tushare shibor --format markdown
```

### 搜索和帮助

```bash
# 列出所有接口
scripts/tushare list

# 搜索接口
scripts/tushare search 股票

# 查看接口详情
scripts/tushare help daily
```

## 错误处理

### Token 未配置或无效

**错误信息**：
```
错误: TUSHARE_TOKEN 未设置
或
错误: 401 Unauthorized
```

**仅在遇到此错误时**，提示用户：

```
需要配置 Tushare Token 才能继续：

1. 访问 https://tushare.pro 注册账号
2. 在个人中心获取 API Token
3. 配置环境变量：
   export TUSHARE_TOKEN="你的Token"

配置完成后，我可以重新执行查询。
```

### 权限不足

**错误信息**：
```
错误: 抱歉，您还没有获得该接口的调取权限
```

**解决方案**：
- 某些接口需要更高积分才能调用
- 访问 https://tushare.pro 查看积分规则
- 完成任务获取积分，或升级到付费版本

### 参数错误

**错误信息**：
```
错误: 参数错误
```

**解决方案**：
1. 使用 `help` 命令查看接口文档：
   ```bash
   scripts/tushare help <接口名>
   ```
2. 检查参数格式：
   - 日期：YYYYMMDD（如 20240228）
   - 股票代码：000001.SZ 格式
3. 确认必填参数是否提供

## 技术分析指南

对于技术指标分析需求，使用以下方法：

### 1. 获取数据

使用 CSV 格式获取数据：

```bash
scripts/tushare daily \
  --ts-code 000725.SZ \
  --start-date 20240201 \
  --end-date 20250228 \
  --format csv
```

### 2. 使用命令行工具分析

推荐使用 `awk` 进行简单的技术指标计算：

```bash
# 计算移动平均线
scripts/tushare daily \
  --ts-code 000725.SZ \
  --start-date 20240201 \
  --end-date 20250228 \
  --format csv | awk -F, 'NR>1 {print $3}' | \
  awk 'BEGIN{ORS=","} {print} END{print "\n"}' | \
  tr ',' '\n' | \
  tail -5 | \
  awk '{sum+=$1; print $1} END{print "MA5:", sum/5}'
```

对于复杂分析，可以：
1. 展示数据给用户，让用户自己分析
2. 使用其他专业分析工具
3. 简单描述分析方法，不直接计算

### 3. 不要做的事

- ❌ 不要编写 Python 脚本
- ❌ 不要创建临时文件存储数据
- ❌ 不要假设用户有 pandas 等依赖
- ✅ 直接展示 CLI 输出
- ✅ 提供分析思路和观察点
- ✅ 使用命令行工具进行简单计算

## 支持的数据类型

| 类别 | 接口数量 | 说明 |
|------|---------|------|
| 股票数据 | 108 | A股行情、财务、交易、筹码、管理层等 |
| 宏观经济 | 21 | GDP、CPI、PPI、利率、货币供应等 |
| 指数专题 | 19 | 各类指数行情、权重、行业分类 |
| 债券专题 | 15 | 可转债、国债、企业债、回购等 |
| 期货数据 | 12 | 期货合约行情、仓单、持仓 |
| 港股数据 | 11 | 港股行情、财务数据 |
| 美股数据 | 9 | 美股行情、财务数据 |
| ETF专题 | 8 | ETF基本信息、行情、份额 |
| 公募基金 | 8 | 基金净值、持仓、分红、规模 |
| 行业经济 | 8 | TMT行业、影视票房等 |
| 大模型语料专题数据 | 6 | 新闻、公告、研报、政策等 |
| 期权数据 | 3 | 期权合约信息、行情 |
| 财富管理 | 2 | 基金销售数据 |
| 外汇数据 | 2 | 外汇基础信息、行情 |
| 现货数据 | 2 | 黄金现货数据 |
| 基金数据 | 1 | 基金相关数据 |
| 其他 | 3 | 其他数据接口 |

## 最佳实践

1. **直接执行**：不要预先检查 Token，直接执行命令
2. **格式选择**：
   - 展示给用户 → 用 `markdown`
   - 需要处理 → 用 `csv`
   - 调试时 → 用 `table`
3. **路径统一**：始终使用 `scripts/tushare` 相对路径
4. **合理设置日期范围**：避免请求过多数据
5. **错误友好提示**：遇到错误时，清楚说明原因和解决方案
6. **避免脚本化**：不要创建 Python 脚本，使用命令行工具或直接展示数据

## 参考资源

- **Tushare 官方文档**：https://tushare.pro/document/2
- **API 测试工具**：https://tushare.pro/document/1
- **积分获取规则**：https://tushare.pro/user/ticket
