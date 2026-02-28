//! 集成测试

use tushare::cli::{parse_args, handle_command};
use tushare::cli::args::{OutputFormat, ParamValue};

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

#[test]
fn test_help_command() {
    let args = vec![
        "tushare".to_string(),
        "help".to_string(),
    ];
    let parsed = parse_args(args).unwrap();
    assert_eq!(parsed.command, "help");
}
