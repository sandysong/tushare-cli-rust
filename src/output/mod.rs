//! 输出格式化模块
//!
//! 负责将数据格式化为不同的输出格式。

pub mod json;
pub mod table;
pub mod csv;
pub mod markdown;

use crate::cli::args::OutputFormat;
use crate::error::TResult;
use std::collections::HashMap;

pub use json::output_json;
pub use table::output_table;
pub use csv::output_csv;
pub use markdown::output_markdown;

/// 根据指定格式输出数据
pub fn output_data(
    data: &[HashMap<String, serde_json::Value>],
    format: OutputFormat,
    pretty: bool,
) -> TResult<()> {
    match format {
        OutputFormat::Json => output_json(data, pretty),
        OutputFormat::Table => output_table(data),
        OutputFormat::Csv => output_csv(data),
        OutputFormat::Markdown => output_markdown(data),
    }
}

/// 获取所有数据的键（字段名）
pub fn get_fields(data: &[HashMap<String, serde_json::Value>]) -> Vec<String> {
    if data.is_empty() {
        return Vec::new();
    }

    let first = &data[0];
    let mut fields: Vec<String> = first.keys().cloned().collect();
    fields.sort();
    fields
}

/// 格式化 JSON 值为字符串
pub fn format_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if f.fract() == 0.0 {
                    format!("{}", n.as_i64().unwrap_or(0))
                } else {
                    format!("{}", f)
                }
            } else {
                n.to_string()
            }
        }
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(_) => "[数组]".to_string(),
        serde_json::Value::Object(_) => "[对象]".to_string(),
    }
}

/// 转义 CSV 值
pub fn escape_csv_value(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace("\"", "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_fields() {
        let data: Vec<HashMap<String, serde_json::Value>> = vec![
            vec![("a".to_string(), json!(1)), ("b".to_string(), json!(2))]
                .into_iter()
                .collect(),
            vec![("c".to_string(), json!(3)), ("d".to_string(), json!(4))]
                .into_iter()
                .collect(),
        ];

        // 第一个对象的键
        let fields = get_fields(&[data[0].clone()]);
        assert_eq!(fields, vec!["a", "b"]);
    }

    #[test]
    fn test_format_value() {
        assert_eq!(format_value(&json!(null)), "");
        assert_eq!(format_value(&json!(true)), "true");
        assert_eq!(format_value(&json!(42)), "42");
        assert_eq!(format_value(&json!(3.14)), "3.14");
        assert_eq!(format_value(&json!("hello")), "hello");
    }

    #[test]
    fn test_escape_csv_value() {
        assert_eq!(escape_csv_value("simple"), "simple");
        assert_eq!(escape_csv_value("with, comma"), "\"with, comma\"");
        assert_eq!(escape_csv_value("with\"quote"), "\"with\"\"quote\"");
        assert_eq!(escape_csv_value("with\nnewline"), "\"with\nnewline\"");
    }
}
