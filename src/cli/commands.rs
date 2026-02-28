//! 命令处理器

use crate::cli::args::ParsedArgs;
use crate::client::TushareClient;
use crate::error::TResult;
use crate::output::output_data;
use std::collections::HashMap;

/// 处理命令
pub async fn handle_command(args: ParsedArgs) -> TResult<()> {
    match args.command.as_str() {
        "help" => {
            show_help(&args);
            Ok(())
        }
        "version" | "--version" | "-v" => {
            show_version();
            Ok(())
        }
        "list" | "ls" => {
            let category = args.positional.first().map(|s| s.as_str());
            list_apis(category);
            Ok(())
        }
        "search" => {
            if let Some(keyword) = args.positional.first() {
                search_apis(keyword);
            } else {
                println!("请提供搜索关键词");
            }
            Ok(())
        }
        _ => {
            // 调用 API
            call_api(args).await
        }
    }
}

/// 显示帮助信息
fn show_help(args: &ParsedArgs) {
    if let Some(api_name) = args.positional.first() {
        // 显示特定 API 的帮助
        show_api_help(api_name);
    } else {
        // 显示通用帮助
        show_general_help();
    }
}

/// 显示通用帮助
fn show_general_help() {
    println!("Tushare Pro CLI - 获取中国金融市场数据的命令行工具");
    println!();
    println!("用法:");
    println!("  tushare <接口名> [选项] [参数...]");
    println!("  tushare <命令> [参数...]");
    println!();
    println!("命令:");
    println!("  help, --help, -h        显示帮助信息");
    println!("  version, --version, -v  显示版本信息");
    println!("  list, ls [类别]         列出所有 API 接口");
    println!("  search <关键词>         搜索 API 接口");
    println!();
    println!("选项:");
    println!("  -f, --format <格式>     输出格式 (json|table|csv|markdown)");
    println!("  -p, --pretty            美化 JSON 输出");
    println!("  -t, --token <token>     API Token");
    println!();
    println!("参数:");
    println!("  --参数名 <值>            API 参数（kebab-case 会自动转换为 snake_case）");
    println!("  --参数名=<值>            API 参数（等号格式）");
    println!();
    println!("常用示例:");
    println!("  # 查看股票基础信息");
    println!("  tushare stock_basic --ts-code 000001.SZ");
    println!();
    println!("  # 查看日线行情");
    println!("  tushare daily --ts-code 000001.SZ --start-date 20240101");
    println!();
    println!("  # 列出所有接口");
    println!("  tushare list");
    println!();
    println!("  # 查看接口帮助");
    println!("  tushare help stock_basic");
    println!();
    println!("  # 搜索接口");
    println!("  tushare search 龙虎榜");
    println!();
    println!("  # 使用不同输出格式");
    println!("  tushare stock_basic --ts-code 000001.SZ --format json --pretty");
    println!("  tushare stock_basic --ts-code 000001.SZ --format csv");
    println!();
    println!("环境变量:");
    println!("  TUSHARE_TOKEN             API Token（推荐设置）");
    println!();
    println!("更多信息:");
    println!("  项目地址: https://github.com/sandysong/tushare-skill");
    println!("  Tushare: https://tushare.pro");
}

/// 显示 API 帮助
fn show_api_help(api_name: &str) {
    use crate::api::{load_api_definitions, find_api_by_name};

    let definitions = load_api_definitions();

    if let Some(api) = find_api_by_name(&definitions, api_name) {
        println!("接口: {}", api.name);
        println!("描述: {}", api.description);
        println!("类别: {}", api.category);

        if let Some(points) = api.requires_points {
            println!("所需积分: {}", points);
        }

        println!();
        println!("参数:");
        if api.parameters.is_empty() {
            println!("  (无参数)");
        } else {
            for param in &api.parameters {
                let required = if param.required { "必选" } else { "可选" };
                println!("  {:<20} {:<6} {}", format!("--{}", param.name), required, param.description);
            }
        }

        println!();
        println!("输出字段:");
        if api.output_fields.is_empty() {
            println!("  (无输出字段)");
        } else {
            for field in &api.output_fields {
                let default = if field.default_show { " *" } else { "" };
                println!("  {:<20} {:<8} {}{}", field.name, field.field_type, field.description, default);
            }
            println!("  (* 表示默认显示字段)");
        }
    } else {
        println!("未找到接口: {}", api_name);
        println!();
        println!("使用 'tushare list' 查看所有可用接口");
        println!("使用 'tushare search <关键词>' 搜索接口");
    }
}

/// 清理描述文本，移除积分和限量相关信息
fn clean_description(desc: &str) -> String {
    let desc = desc
        .split("限量：")
        .next()
        .unwrap_or(desc)
        .split("积分：")
        .next()
        .unwrap_or(desc)
        .split("权限：")
        .next()
        .unwrap_or(desc)
        .split("提示：")
        .next()
        .unwrap_or(desc)
        .split("更新：")
        .next()
        .unwrap_or(desc)
        .split("历史：")
        .next()
        .unwrap_or(desc)
        .trim();

    // 移除末尾的句号和多余空格
    let desc = desc.trim_end_matches('.').trim();

    // 限制长度（使用字符边界）
    if desc.chars().count() > 60 {
        let chars: Vec<char> = desc.chars().take(57).collect();
        format!("{}...", chars.iter().collect::<String>())
    } else {
        desc.to_string()
    }
}

/// 显示版本信息
fn show_version() {
    println!("Tushare CLI 版本 {}", env!("CARGO_PKG_VERSION"));
    println!("Rust 版本 {}", env!("CARGO_PKG_RUST_VERSION"));

    // 获取 Rust 编译版本信息
    if let Some(rust_version) = option_env!("RUSTC_VERSION") {
        println!("编译器版本 {}", rust_version);
    }

    println!();
    println!("Copyright (c) 2024 Sandy Song");
    println!("Licensed under the MIT License");
    println!();
    println!("项目地址: https://github.com/sandysong/tushare-skill");
}

/// 列出所有 API
fn list_apis(category: Option<&str>) {
    use crate::api::{load_api_definitions, get_categories};

    let definitions = load_api_definitions();

    if definitions.is_empty() {
        println!("(无法加载 API 定义)");
        return;
    }

    if let Some(cat) = category {
        // 按类别过滤
        let filtered: Vec<_> = definitions
            .values()
            .filter(|api| api.category == cat)
            .collect();

        if filtered.is_empty() {
            println!("类别 '{}' 下没有找到 API 接口", cat);
            println!();
            println!("可用类别:");
            for c in get_categories() {
                println!("  - {}", c);
            }
        } else {
            println!("类别: {} (共 {} 个接口)", cat, filtered.len());
            println!();
            for api in filtered {
                // 清理描述，移除积分和限量信息
                let description = clean_description(&api.description);
                println!("  {:<25} {}", api.name, description);
            }
        }
    } else {
        // 列出所有类别及其接口数量
        let mut category_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for api in definitions.values() {
            *category_counts.entry(&api.category).or_insert(0) += 1;
        }

        println!("所有 API 接口 (共 {} 个)", definitions.len());
        println!();
        for cat in get_categories() {
            if let Some(&count) = category_counts.get(cat) {
                println!("  {:<15} {}", cat, count);
            }
        }
        println!();
        println!("使用 'tushare list <类别>' 查看具体接口");
        println!("使用 'tushare help <接口名>' 查看接口详情");
        println!("使用 'tushare search <关键词>' 搜索接口");
    }
}

/// 搜索 API
fn search_apis(keyword: &str) {
    use crate::api::{load_api_definitions, search_apis};

    let definitions = load_api_definitions();

    if definitions.is_empty() {
        println!("(无法加载 API 定义)");
        return;
    }

    let results = search_apis(&definitions, keyword);

    if results.is_empty() {
        println!("未找到包含 '{}' 的 API 接口", keyword);
        println!();
        println!("提示:");
        println!("  - 尝试使用更通用的关键词");
        println!("  - 使用 'tushare list' 查看所有接口");
    } else {
        println!("搜索 '{}' (找到 {} 个结果):", keyword, results.len());
        println!();
        for api in results {
            // 清理描述
            let description = clean_description(&api.description);
            println!("  {:<25} {}", api.name, description);
        }
    }
}

/// 调用 API
async fn call_api(args: ParsedArgs) -> TResult<()> {
    use crate::api::{find_api_by_name, load_api_definitions};

    // 检查 API 是否存在
    let definitions = load_api_definitions();
    if let Some(_api_def) = find_api_by_name(&definitions, &args.command) {
        // API 存在，继续调用
    }

    // 创建客户端
    let mut client = TushareClient::new()?;

    // 设置 Token
    if let Some(token) = &args.options.token {
        client.set_token(token.clone());
    } else {
        match client.load_token_from_env() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("错误: {}", e);
                eprintln!();
                eprintln!("请设置 API Token:");
                eprintln!("  1. 设置环境变量: export TUSHARE_TOKEN=\"your_token\"");
                eprintln!("  2. 或使用参数: tushare --token \"your_token\" <接口名>");
                eprintln!();
                eprintln!("获取 Token: https://tushare.pro/register");
                return Err(e);
            }
        }
    }

    // 调用 API
    let response = match client
        .call(&args.command, Some(args.params_to_json()), None)
        .await
    {
        Ok(r) => r,
        Err(crate::error::TushareError::ApiError { code, msg }) => {
            eprintln!("API 调用失败 (错误码: {}): {}", code, msg);
            if code == -10000 {
                eprintln!();
                eprintln!("可能的原因:");
                eprintln!("  - Token 无效或已过期");
                eprintln!("  - 账户积分不足");
                eprintln!("  - 接口权限未开通");
                eprintln!();
                eprintln!("请检查: https://tushare.pro/user/token");
            } else if code == -10001 {
                eprintln!();
                eprintln!("可能的原因:");
                eprintln!("  - API 接口名称错误");
                eprintln!("  - 接口已停用或下线");
                eprintln!();
                eprintln!("使用 'tushare list' 查看所有可用接口");
            }
            return Err(crate::error::TushareError::ApiError { code, msg });
        }
        Err(e) => return Err(e),
    };

    // 输出结果
    let objects = response.to_objects();
    output_data(&objects, args.options.format, args.options.pretty)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::args::{OutputFormat, Options};

    #[test]
    fn test_show_general_help() {
        show_general_help();
        // 如果没有 panic，测试通过
    }

    #[test]
    fn test_show_version() {
        show_version();
        // 如果没有 panic，测试通过
    }

    #[test]
    fn test_list_apis() {
        list_apis(None);
        list_apis(Some("股票数据"));
        // 如果没有 panic，测试通过
    }

    #[test]
    fn test_search_apis() {
        search_apis("龙虎榜");
        // 如果没有 panic，测试通过
    }
}
