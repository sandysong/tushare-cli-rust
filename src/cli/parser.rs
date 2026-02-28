//! CLI 参数解析器

use crate::cli::args::{OutputFormat, ParsedArgs, ParamValue, Options};
use crate::error::{TushareError, TResult};

/// 将 kebab-case 转换为 snake_case
pub fn kebab_to_snake(s: &str) -> String {
    s.replace('-', "_")
}

/// 将 snake_case 转换为 kebab-case
pub fn snake_to_kebab(s: &str) -> String {
    s.replace('_', "-")
}

/// 解析命令行参数
pub fn parse_args(args: Vec<String>) -> TResult<ParsedArgs> {
    let mut iter = args.iter().peekable();
    iter.next(); // 跳过程序名

    let mut parsed = ParsedArgs::new(String::new());
    let mut options = Options::default();

    // 解析选项和参数
    while let Some(arg) = iter.next() {
        // 处理选项
        if arg.starts_with('-') {
            // 短选项
            if arg.starts_with("--") {
                // 长选项
                let opt = &arg[2..];
                match opt {
                    "help" => options.help = true,
                    "version" => options.version = true,
                    "format" => {
                        if let Some(fmt) = iter.next() {
                            if let Some(fmt_enum) = OutputFormat::from_str(fmt) {
                                options.format = fmt_enum;
                            } else {
                                return Err(TushareError::ParseError(format!(
                                    "无效的输出格式: {}",
                                    fmt
                                )));
                            }
                        }
                    }
                    "pretty" => options.pretty = true,
                    "token" => {
                        if let Some(token) = iter.next() {
                            options.token = Some(token.clone());
                        }
                    }
                    _ => {
                        // 可能是 API 参数（格式：--param value 或 --param=value）
                        if let Some(eq_idx) = opt.find('=') {
                            let key = kebab_to_snake(&opt[..eq_idx]);
                            let value = &opt[eq_idx + 1..];
                            parsed.add_param(key, ParamValue::from_str(value.to_string()));
                        } else if let Some(value) = iter.peek() {
                            // 检查下一个是否是值（不是选项）
                            if !value.starts_with('-') {
                                let value = iter.next().unwrap();
                                parsed.add_param(kebab_to_snake(opt), ParamValue::from_str(value.clone()));
                            }
                        } else {
                            // 标志参数，视为布尔值 true
                            parsed.add_param(kebab_to_snake(opt), ParamValue::Boolean(true));
                        }
                    }
                }
            } else {
                // 短选项
                let chars: Vec<char> = arg.chars().collect();
                if chars.len() > 1 {
                    let opt = chars[1];
                    match opt {
                        'h' => options.help = true,
                        'v' => options.version = true,
                        'f' => {
                            if let Some(fmt) = iter.next() {
                                if let Some(fmt_enum) = OutputFormat::from_str(fmt) {
                                    options.format = fmt_enum;
                                }
                            }
                        }
                        'p' => options.pretty = true,
                        't' => {
                            if let Some(token) = iter.next() {
                                options.token = Some(token.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        } else {
            // 位置参数
            if parsed.command.is_empty() {
                parsed.command = arg.clone();
            } else {
                parsed.add_positional(arg.clone());
            }
        }
    }

    // 如果没有命令，根据选项设置默认命令
    if parsed.command.is_empty() {
        if options.version {
            parsed.command = "version".to_string();
        } else if options.help {
            parsed.command = "help".to_string();
        } else {
            parsed.command = "help".to_string();
        }
    }

    parsed.options = options;

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kebab_to_snake() {
        assert_eq!(kebab_to_snake("ts-code"), "ts_code");
        assert_eq!(kebab_to_snake("start-date"), "start_date");
        assert_eq!(kebab_to_snake("list-status"), "list_status");
        assert_eq!(kebab_to_snake("already_snake"), "already_snake");
    }

    #[test]
    fn test_snake_to_kebab() {
        assert_eq!(snake_to_kebab("ts_code"), "ts-code");
        assert_eq!(snake_to_kebab("start_date"), "start-date");
        assert_eq!(snake_to_kebab("list_status"), "list-status");
    }

    #[test]
    fn test_parse_simple_command() {
        let args = vec![
            "tushare".to_string(),
            "stock_basic".to_string(),
        ];
        let parsed = parse_args(args).unwrap();
        assert_eq!(parsed.command, "stock_basic");
    }

    #[test]
    fn test_parse_with_params() {
        let args = vec![
            "tushare".to_string(),
            "stock_basic".to_string(),
            "--ts-code".to_string(),
            "000001.SZ".to_string(),
        ];
        let parsed = parse_args(args).unwrap();
        assert_eq!(parsed.command, "stock_basic");
        assert_eq!(
            parsed.get_param("ts_code"),
            Some(&ParamValue::String("000001.SZ".to_string()))
        );
    }

    #[test]
    fn test_parse_with_equals() {
        let args = vec![
            "tushare".to_string(),
            "stock_basic".to_string(),
            "--limit=10".to_string(),
        ];
        let parsed = parse_args(args).unwrap();
        assert_eq!(parsed.get_param("limit"), Some(&ParamValue::Number(10.0)));
    }

    #[test]
    fn test_parse_options() {
        let args = vec![
            "tushare".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--pretty".to_string(),
            "stock_basic".to_string(),
        ];
        let parsed = parse_args(args).unwrap();
        assert_eq!(parsed.options.format, OutputFormat::Json);
        assert!(parsed.options.pretty);
    }
}
