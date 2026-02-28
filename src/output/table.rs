//! 表格格式化输出

use crate::output::{format_value, get_fields};
use crate::error::TResult;
use comfy_table::{presets::UTF8_FULL, Table};
use std::collections::HashMap;

/// 以表格格式输出数据
pub fn output_table(data: &[HashMap<String, serde_json::Value>]) -> TResult<()> {
    if data.is_empty() {
        println!("(无数据)");
        return Ok(());
    }

    let fields = get_fields(data);

    // 限制显示行数
    let max_rows = 20;
    let display_data: Vec<_> = data.iter().take(max_rows).cloned().collect();

    // 创建表格
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(fields.clone());

    // 添加数据行
    for row in &display_data {
        let mut values = Vec::new();
        for field in &fields {
            let value = row.get(field).map(format_value).unwrap_or_default();
            values.push(value);
        }
        table.add_row(values);
    }

    // 如果数据被截断，添加提示
    if data.len() > max_rows {
        table.add_row(vec![format!(
            "... (共 {} 行，仅显示前 {} 行)",
            data.len(),
            max_rows
        )]);
    }

    println!("{}", table);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_output_table() {
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

        let result = output_table(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_empty_table() {
        let data: Vec<HashMap<String, serde_json::Value>> = Vec::new();
        let result = output_table(&data);
        assert!(result.is_ok());
    }
}
