//! Tushare API 客户端

use crate::client::request::TushareRequest;
use crate::client::response::TushareResponse;
use crate::error::{TushareError, TResult};
use reqwest::Client;

const TUSHARE_API_URL: &str = "https://api.tushare.pro";

/// Tushare API 客户端
#[derive(Debug, Clone)]
pub struct TushareClient {
    /// HTTP 客户端
    client: Client,
    /// API Token
    token: Option<String>,
}

impl TushareClient {
    /// 创建新的 API 客户端
    pub fn new() -> TResult<Self> {
        Ok(Self {
            client: Client::new(),
            token: None,
        })
    }

    /// 使用指定 Token 创建客户端
    pub fn with_token(token: String) -> TResult<Self> {
        Ok(Self {
            client: Client::new(),
            token: Some(token),
        })
    }

    /// 设置 Token
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// 获取 Token
    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// 从环境变量加载 Token
    pub fn load_token_from_env(&mut self) -> TResult<()> {
        let token = std::env::var("TUSHARE_TOKEN")
            .map_err(|_| TushareError::TokenNotConfigured)?;
        self.token = Some(token);
        Ok(())
    }

    /// 调用 Tushare API
    pub async fn call(&self, api_name: &str, params: Option<serde_json::Value>, fields: Option<String>) -> TResult<TushareResponse> {
        // 获取 Token
        let token = self.get_token().ok_or(TushareError::TokenNotConfigured)?;

        // 构建请求
        let request = TushareRequest::new(api_name.to_string(), token.to_string())
            .with_params(params.unwrap_or_default())
            .with_fields(fields.unwrap_or_default());

        // 发送请求
        let response = self
            .client
            .post(TUSHARE_API_URL)
            .json(&request)
            .send()
            .await?;

        // 解析响应
        let tushare_response: TushareResponse = response.json().await?;

        // 检查 API 错误
        if !tushare_response.is_success() {
            return Err(TushareError::ApiError {
                code: tushare_response.code,
                msg: tushare_response.msg.clone(),
            });
        }

        Ok(tushare_response)
    }

    /// 调用 Tushare API（同步版本）
    pub fn call_sync(&self, api_name: &str, params: Option<serde_json::Value>, fields: Option<String>) -> TResult<TushareResponse> {
        // 获取 Token
        let token = self.get_token().ok_or(TushareError::TokenNotConfigured)?;

        // 构建请求
        let request = TushareRequest::new(api_name.to_string(), token.to_string())
            .with_params(params.unwrap_or_default())
            .with_fields(fields.unwrap_or_default());

        // 使用 runtime block
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            // 发送请求
            let response = self
                .client
                .post(TUSHARE_API_URL)
                .json(&request)
                .send()
                .await?;

            // 解析响应
            let tushare_response: TushareResponse = response.json().await?;

            // 检查 API 错误
            if !tushare_response.is_success() {
                return Err(TushareError::ApiError {
                    code: tushare_response.code,
                    msg: tushare_response.msg.clone(),
                });
            }

            Ok(tushare_response)
        })
    }
}

impl Default for TushareClient {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = TushareClient::new();
        assert!(client.is_ok());
        assert!(client.unwrap().get_token().is_none());
    }

    #[test]
    fn test_client_with_token() {
        let client = TushareClient::with_token("test_token".to_string()).unwrap();
        assert_eq!(client.get_token(), Some("test_token"));
    }

    #[test]
    fn test_set_token() {
        let mut client = TushareClient::new().unwrap();
        assert!(client.get_token().is_none());

        client.set_token("new_token".to_string());
        assert_eq!(client.get_token(), Some("new_token"));
    }

    #[test]
    fn test_request_building() {
        let client = TushareClient::with_token("test_token".to_string()).unwrap();
        let request = TushareRequest::new("stock_basic".to_string(), "test_token".to_string());

        assert_eq!(request.api_name, "stock_basic");
        assert_eq!(request.token, "test_token");
    }
}
