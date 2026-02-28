//! CLI 参数类型定义

use serde::{Deserialize, Serialize};

/// 输出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// JSON 格式
    Json,
    /// 表格格式
    Table,
    /// CSV 格式
    Csv,
    /// Markdown 格式
    Markdown,
}

impl OutputFormat {
    /// 从字符串解析输出格式
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(OutputFormat::Json),
            "table" => Some(OutputFormat::Table),
            "csv" => Some(OutputFormat::Csv),
            "markdown" => Some(OutputFormat::Markdown),
            _ => None,
        }
    }

    /// 获取格式名称
    pub fn as_str(&self) -> &str {
        match self {
            OutputFormat::Json => "json",
            OutputFormat::Table => "table",
            OutputFormat::Csv => "csv",
            OutputFormat::Markdown => "markdown",
        }
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Table
    }
}

/// 参数值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParamValue {
    /// 字符串值
    String(String),
    /// 数字值
    Number(f64),
    /// 布尔值
    Boolean(bool),
}

impl ParamValue {
    /// 从字符串解析参数值
    pub fn from_str(s: String) -> Self {
        // 尝试解析为数字
        if let Ok(n) = s.parse::<f64>() {
            return ParamValue::Number(n);
        }

        // 尝试解析为布尔值
        match s.to_lowercase().as_str() {
            "true" => return ParamValue::Boolean(true),
            "false" => return ParamValue::Boolean(false),
            _ => {}
        }

        // 默认为字符串
        ParamValue::String(s)
    }
}

impl From<String> for ParamValue {
    fn from(s: String) -> Self {
        ParamValue::from_str(s)
    }
}

impl From<&str> for ParamValue {
    fn from(s: &str) -> Self {
        ParamValue::from_str(s.to_string())
    }
}

impl From<f64> for ParamValue {
    fn from(n: f64) -> Self {
        ParamValue::Number(n)
    }
}

impl From<bool> for ParamValue {
    fn from(b: bool) -> Self {
        ParamValue::Boolean(b)
    }
}

/// CLI 选项
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// 输出格式
    pub format: OutputFormat,
    /// 是否美化输出（仅 JSON）
    pub pretty: bool,
    /// API Token（覆盖环境变量）
    pub token: Option<String>,
    /// 显示帮助
    pub help: bool,
    /// 显示版本
    pub version: bool,
}

/// 解析后的参数
#[derive(Debug, Clone)]
pub struct ParsedArgs {
    /// 命令名称（API 名称或内置命令）
    pub command: String,
    /// API 参数
    pub params: Vec<(String, ParamValue)>,
    /// 选项
    pub options: Options,
    /// 位置参数
    pub positional: Vec<String>,
}

impl ParsedArgs {
    /// 创建新的解析后参数
    pub fn new(command: String) -> Self {
        Self {
            command,
            params: Vec::new(),
            options: Options::default(),
            positional: Vec::new(),
        }
    }

    /// 添加参数
    pub fn add_param(&mut self, key: String, value: ParamValue) {
        self.params.push((key, value));
    }

    /// 添加位置参数
    pub fn add_positional(&mut self, arg: String) {
        self.positional.push(arg);
    }

    /// 获取参数值
    pub fn get_param(&self, key: &str) -> Option<&ParamValue> {
        self.params.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// 将参数转换为 JSON 对象
    pub fn params_to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for (key, value) in &self.params {
            let json_value = match value {
                ParamValue::String(s) => serde_json::Value::String(s.clone()),
                ParamValue::Number(n) => {
                    if n.fract() == 0.0 {
                        serde_json::Value::Number(serde_json::Number::from(*n as i64))
                    } else {
                        serde_json::Value::Number(
                            serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0)),
                        )
                    }
                }
                ParamValue::Boolean(b) => serde_json::Value::Bool(*b),
            };
            map.insert(key.clone(), json_value);
        }
        serde_json::Value::Object(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert_eq!(OutputFormat::from_str("json"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("JSON"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("table"), Some(OutputFormat::Table));
        assert_eq!(OutputFormat::from_str("csv"), Some(OutputFormat::Csv));
        assert_eq!(OutputFormat::from_str("markdown"), Some(OutputFormat::Markdown));
        assert_eq!(OutputFormat::from_str("invalid"), None);
    }

    #[test]
    fn test_param_value_from_str() {
        assert_eq!(ParamValue::from_str("123".to_string()), ParamValue::Number(123.0));
        assert_eq!(ParamValue::from_str("true".to_string()), ParamValue::Boolean(true));
        assert_eq!(ParamValue::from_str("false".to_string()), ParamValue::Boolean(false));
        assert_eq!(
            ParamValue::from_str("hello".to_string()),
            ParamValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_parsed_args() {
        let mut args = ParsedArgs::new("stock_basic".to_string());
        args.add_param("ts_code".to_string(), "000001.SZ".into());
        args.add_param("limit".to_string(), 10.0.into());

        assert_eq!(args.get_param("ts_code"), Some(&ParamValue::String("000001.SZ".to_string())));
        assert_eq!(args.get_param("limit"), Some(&ParamValue::Number(10.0)));
    }

    #[test]
    fn test_params_to_json() {
        let mut args = ParsedArgs::new("test".to_string());
        args.add_param("str_val".to_string(), "hello".into());
        args.add_param("num_val".to_string(), 42.0.into());
        args.add_param("bool_val".to_string(), true.into());

        let json = args.params_to_json();
        assert_eq!(json["str_val"], "hello");
        assert_eq!(json["num_val"], 42);
        assert_eq!(json["bool_val"], true);
    }
}
