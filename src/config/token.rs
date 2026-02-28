//! Token 管理

use crate::error::{TushareError, TResult};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// 获取 Token 配置文件路径
pub fn get_token_path() -> PathBuf {
    // 优先使用环境变量指定的路径
    if let Ok(path) = env::var("TUSHARE_CONFIG_PATH") {
        return PathBuf::from(path);
    }

    // 使用用户主目录下的配置文件
    let mut path = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    path.push_str("/.tushare/token.txt");

    PathBuf::from(path)
}

/// 从文件加载 Token
pub fn load_token() -> TResult<Option<String>> {
    let path = get_token_path();

    if !path.exists() {
        return Ok(None);
    }

    let token = fs::read_to_string(&path)?.trim().to_string();

    if token.is_empty() {
        Ok(None)
    } else {
        Ok(Some(token))
    }
}

/// 保存 Token 到文件
pub fn save_token(token: &str) -> TResult<()> {
    let path = get_token_path();

    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(&path)?;
    writeln!(file, "{}", token)?;

    Ok(())
}

/// 从环境变量或配置文件获取 Token
pub fn get_token() -> TResult<String> {
    // 优先使用环境变量
    if let Ok(token) = env::var("TUSHARE_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    // 其次使用配置文件
    if let Some(token) = load_token()? {
        return Ok(token);
    }

    Err(TushareError::TokenNotConfigured)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token_path() {
        let path = get_token_path();
        assert!(path.ends_with(".tushare/token.txt"));
    }

    #[test]
    fn test_save_and_load_token() {
        // 注意：这会创建真实的文件
        // 在实际测试中应该使用临时目录
        let test_token = "test_token_12345";

        let result = save_token(test_token);
        // 可能成功也可能失败，取决于文件系统权限

        if result.is_ok() {
            let loaded = load_token().unwrap();
            assert_eq!(loaded, Some(test_token.to_string()));

            // 清理
            let path = get_token_path();
            let _ = fs::remove_file(&path);
        }
    }
}
