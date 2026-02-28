//! Markdown 格式化输出

use crate::output::{format_value, get_fields};
use crate::error::TResult;
use std::collections::HashMap;
use std::io::Write;

/// 以 Markdown 表格格式输出数据
pub fn output_markdown(data: &[HashMap<String, serde_json::Value>]) -> TResult<()> {
    if data.is_empty() {
        println!("(无数据)");
        return Ok(());
    }

    let fields = get_fields(data);

    // 限制显示行数
    let max_rows = 100;
    let display_data: Vec<_> = data.iter().take(max_rows).cloned().collect();

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    // 计算列宽
    let mut widths: Vec<usize> = fields.iter().map(|f| f.len()).collect();
    for row in &display_data {
        for (i, field) in fields.iter().enumerate() {
            let value = row.get(field).map(format_value).unwrap_or_default();
            let value_len = if value.len() > 30 {
                30
            } else {
                value.len()
            };
            widths[i] = widths[i].max(value_len);
        }
    }

    // 输出表头
    let header: Vec<String> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| format!("{:<width$}", f, width = widths[i]))
        .collect();
    writeln!(handle, "| {} |", header.join(" | "))
        .map_err(|e| crate::error::TushareError::OutputError(format!("写入输出失败: {}", e)))?;

    // 输出分隔行
    let separator: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
    writeln!(handle, "|{}|", separator.join("|"))
        .map_err(|e| crate::error::TushareError::OutputError(format!("写入输出失败: {}", e)))?;

    // 输出数据行
    for row in &display_data {
        let values: Vec<String> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let value = row.get(f).map(format_value).unwrap_or_default();
                let truncated = if value.len() > 30 {
                    format!("{}...", &value[..27])
                } else {
                    value
                };
                format!("{:<width$}", truncated, width = widths[i])
            })
            .collect();

        writeln!(handle, "| {} |", values.join(" | "))
            .map_err(|e| crate::error::TushareError::OutputError(format!("写入输出失败: {}", e)))?;
    }

    // 如果数据被截断，添加提示
    if data.len() > max_rows {
        writeln!(
            handle,
            "\n... (共 {} 行，仅显示前 {} 行)",
            data.len(),
            max_rows
        )
        .map_err(|e| crate::error::TushareError::OutputError(format!("写入输出失败: {}", e)))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_output_markdown() {
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

        let result = output_markdown(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_empty_markdown() {
        let data: Vec<HashMap<String, serde_json::Value>> = Vec::new();
        let result = output_markdown(&data);
        assert!(result.is_ok());
    }
}
