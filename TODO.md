# Tushare CLI Rust - 待完善功能清单

## 🔴 重要功能

### 1. 版本命令修复
**问题**: `-v` 参数被解析为显示帮助，而不是版本信息
**优先级**: 高
**位置**: `src/cli/parser.rs:29`

**修复方案**:
```rust
// 需要区分 `-v` 作为独立命令和作为选项的情况
// 当 `-v` 单独出现时显示版本，而不是帮助
```

### 2. API 定义加载
**问题**: `list` 和 `search` 命令返回"正在开发中"
**优先级**: 高
**位置**: `src/api/definitions.rs`

**实现步骤**:
1. 从 TypeScript 项目导出 API 定义为 JSON
2. 在 Rust 中嵌入 JSON 文件
3. 实现 `load_api_definitions()` 函数
4. 更新 `list` 和 `search` 命令使用真实数据

**命令**:
```bash
cd /Users/songqi/Work/quant/tushare-cli-rust
./scripts/generate-definitions.sh
```

### 3. API 详细帮助信息
**问题**: `help <api_name>` 显示"正在开发中"
**优先级**: 中
**位置**: `src/cli/commands.rs:show_api_help()`

**实现内容**:
- 显示 API 描述
- 列出所有参数及类型
- 标注必选/可选参数
- 显示输出字段说明

## 🟡 增强功能

### 4. 更好的错误提示
**优先级**: 中
**位置**: `src/cli/commands.rs:call_api()`

**改进**:
- API 调用失败时显示具体原因
- 参数验证错误时提示正确格式
- Token 错误时提示如何设置

### 5. 数据分页显示
**优先级**: 低
**位置**: `src/output/table.rs`, `src/output/markdown.rs`

**实现**:
- 支持翻页查看大量数据
- 添加 `--page` 和 `--per-page` 选项
- 类似 `less` 的交互式浏览

### 6. 配置文件支持
**优先级**: 低
**位置**: `src/config/`

**实现**:
- `~/.tushare/config.toml` 配置文件
- 支持默认输出格式、默认 Token 等
- 配置文件热重载

## 🟢 可选优化

### 7. UPX 二进制压缩
**预期效果**: 1.7MB → ~1MB
**命令**:
```bash
brew install upx
./scripts/strip-binary.sh target/release/tushare
```

### 8. 跨平台编译
**目标平台**:
- macOS (ARM64 + x64)
- Linux (x64)
- Windows (x64)

**实现**: GitHub Actions 自动构建

### 9. 安装脚本
**文件**: `scripts/install.sh`
**功能**:
- 自动下载对应平台的二进制
- 安装到 `/usr/local/bin`
- 设置权限

### 10. Shell 自动补全
**支持**:
- Bash completion
- Zsh completion
- Fish completion

### 11. 更多测试
**添加**:
- 边界情况测试
- 错误处理测试
- 性能基准测试

## 📋 实现优先级

### 第一批（核心功能）
1. ✅ ~~版本命令修复~~
2. ✅ ~~API 定义加载~~
3. ✅ ~~API 详细帮助~~

### 第二批（用户体验）
4. 更好的错误提示
5. 数据分页显示
6. 配置文件支持

### 第三批（工程化）
7. UPX 压缩
8. 跨平台编译
9. 安装脚本

### 第四批（锦上添花）
10. Shell 自动补全
11. 更多测试

## 🚀 快速启动

要实现最重要的 3 个功能，按以下顺序：

```bash
# 1. 修复版本命令
# 编辑 src/cli/parser.rs

# 2. 加载 API 定义
cd /Users/songqi/Work/quant/tushare-cli-rust
./scripts/generate-definitions.sh
# 然后更新 src/api/definitions.rs

# 3. 实现 API 帮助
# 编辑 src/cli/commands.rs:show_api_help()
```

## 📝 注意事项

- 当前项目**完全可以使用**，所有核心 API 调用功能正常
- 待完善的功能主要是**用户体验增强**
- 不影响基本的 API 查询功能
