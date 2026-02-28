//! CSV 格式化输出

use crate::output::{escape_csv_value, format_value, get_fields};
use crate::error::TResult;
use std::collections::HashMap;
use std::io::Write;

/// 以 CSV 格式输出数据
pub fn output_csv(data: &[HashMap<String, serde_json::Value>]) -> TResult<()> {
    if data.is_empty() {
        println!("(无数据)");
        return Ok(());
    }

    let fields = get_fields(data);
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    // 输出表头
    let header: Vec<String> = fields.iter().map(|f| escape_csv_value(f)).collect();
    writeln!(handle, "{}", header.join(","))
        .map_err(|e| crate::error::TushareError::OutputError(format!("写入输出失败: {}", e)))?;

    // 输出数据行
    for row in data {
        let values: Vec<String> = fields
            .iter()
            .map(|f| {
                let value = row.get(f).map(format_value).unwrap_or_default();
                escape_csv_value(&value)
            })
            .collect();

        writeln!(handle, "{}", values.join(","))
            .map_err(|e| crate::error::TushareError::OutputError(format!("写入输出失败: {}", e)))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_output_csv() {
        let data = vec![
            vec![
                ("name".to_string(), json!("Alice")),
                ("age".to_string(), json!(25)),
            ]
            .into_iter()
            .collect(),
            vec![
                ("name".to_string(), json!("Bob")),
                ("age".to_string(), json!(30)),
            ]
            .into_iter()
            .collect(),
        ];

        let result = output_csv(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_csv_with_special_chars() {
        let data = vec![vec![
            ("name".to_string(), json!("Alice, Bob")),
            ("note".to_string(), json!("He said \"hello\"")),
        ]
        .into_iter()
        .collect()];

        let result = output_csv(&data);
        assert!(result.is_ok());
    }
}
