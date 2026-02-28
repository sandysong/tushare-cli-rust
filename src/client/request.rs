//! Tushare API 请求结构

use serde::{Deserialize, Serialize};

/// Tushare API 请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TushareRequest {
    /// API 名称
    #[serde(rename = "api_name")]
    pub api_name: String,

    /// Token
    pub token: String,

    /// API 参数
    pub params: Option<serde_json::Value>,

    /// 输出字段
    pub fields: Option<String>,
}

impl TushareRequest {
    /// 创建新的 API 请求
    pub fn new(api_name: String, token: String) -> Self {
        Self {
            api_name,
            token,
            params: None,
            fields: None,
        }
    }

    /// 设置参数
    pub fn with_params(mut self, params: serde_json::Value) -> Self {
        self.params = Some(params);
        self
    }

    /// 设置字段
    pub fn with_fields(mut self, fields: String) -> Self {
        self.fields = Some(fields);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_creation() {
        let req = TushareRequest::new("stock_basic".to_string(), "test_token".to_string());
        assert_eq!(req.api_name, "stock_basic");
        assert_eq!(req.token, "test_token");
        assert!(req.params.is_none());
        assert!(req.fields.is_none());
    }

    #[test]
    fn test_request_with_params() {
        let req = TushareRequest::new("stock_basic".to_string(), "test_token".to_string())
            .with_params(json!({"ts_code": "000001.SZ"}))
            .with_fields("ts_code,name".to_string());

        assert_eq!(req.params.unwrap()["ts_code"], "000001.SZ");
        assert_eq!(req.fields.unwrap(), "ts_code,name");
    }

    #[test]
    fn test_request_serialize() {
        let req = TushareRequest::new("stock_basic".to_string(), "test_token".to_string())
            .with_params(json!({"limit": 10}));

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("api_name"));
        assert!(json.contains("stock_basic"));
    }
}
