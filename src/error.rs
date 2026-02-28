//! 错误类型定义
//!
//! 定义了 Tushare CLI 中所有可能出现的错误类型。

use thiserror::Error;

/// Tushare CLI 错误类型
#[derive(Error, Debug)]
pub enum TushareError {
    /// HTTP 请求错误
    #[error("HTTP 请求失败: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON 序列化/反序列化错误
    #[error("JSON 处理失败: {0}")]
    JsonError(#[from] serde_json::Error),

    /// IO 错误
    #[error("IO 操作失败: {0}")]
    IoError(#[from] std::io::Error),

    /// API 返回错误
    #[error("API 返回错误 (code={code}): {msg}")]
    ApiError { code: i32, msg: String },

    /// Token 未配置
    #[error("Token 未配置，请设置 TUSHARE_TOKEN 环境变量或使用 --token 参数")]
    TokenNotConfigured,

    /// 参数解析错误
    #[error("参数解析错误: {0}")]
    ParseError(String),

    /// API 定义未找到
    #[error("API 定义未找到: {0}")]
    ApiNotFound(String),

    /// 参数验证错误
    #[error("参数验证失败: {0}")]
    ValidationError(String),

    /// 输出格式错误
    #[error("输出格式错误: {0}")]
    OutputError(String),
}

/// TResult 类型别名
pub type TResult<T> = Result<T, TushareError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = TushareError::ApiError {
            code: -1,
            msg: "测试错误".to_string(),
        };
        assert_eq!(format!("{}", err), "API 返回错误 (code=-1): 测试错误");
    }

    #[test]
    fn test_token_not_configured() {
        let err = TushareError::TokenNotConfigured;
        assert!(format!("{}", err).contains("Token 未配置"));
    }
}
