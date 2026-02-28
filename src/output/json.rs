//! JSON 格式化输出

use crate::error::TResult;
use std::collections::HashMap;
use std::io::Write;

/// 以 JSON 格式输出数据
pub fn output_json(data: &[HashMap<String, serde_json::Value>], pretty: bool) -> TResult<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    let json = if pretty {
        serde_json::to_string_pretty(data)
    } else {
        serde_json::to_string(data)
    };

    let json_str = json.map_err(|e| crate::error::TushareError::OutputError(format!("JSON 序列化失败: {}", e)))?;
    writeln!(handle, "{}", json_str)
        .map_err(|e| crate::error::TushareError::OutputError(format!("写入输出失败: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_output_json() {
        let data = vec![vec![("key".to_string(), json!("value"))]
            .into_iter()
            .collect()];

        let result = output_json(&data, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_json_pretty() {
        let data = vec![vec![("key".to_string(), json!("value"))]
            .into_iter()
            .collect()];

        let result = output_json(&data, true);
        assert!(result.is_ok());
    }
}
