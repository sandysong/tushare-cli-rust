//! Tushare API 响应结构

use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/// Tushare API 响应
#[derive(Debug, Clone, Deserialize)]
pub struct TushareResponse {
    /// 请求 ID
    #[serde(rename = "request_id")]
    pub request_id: String,

    /// 响应码 (0 表示成功)
    pub code: i32,

    /// 响应消息
    pub msg: String,

    /// 响应数据
    pub data: Option<ResponseData>,
}

/// 响应数据
#[derive(Debug, Clone)]
pub struct ResponseData {
    /// 字段名列表
    pub fields: Vec<String>,

    /// 数据项（二维数组）
    pub items: Vec<Vec<serde_json::Value>>,
}

/// Tushare API 错误响应
#[derive(Debug, Clone, Deserialize)]
pub struct TushareErrorResponse {
    /// 请求 ID
    #[serde(rename = "request_id")]
    pub request_id: String,

    /// 错误码
    pub code: i32,

    /// 错误消息
    pub msg: String,
}

// 自定义反序列化实现
impl<'de> Deserialize<'de> for ResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawResponseData {
            fields: Option<serde_json::Value>,
            items: Option<Vec<Vec<serde_json::Value>>>,
        }

        let raw = RawResponseData::deserialize(deserializer)?;

        // 处理 fields 字段（可能是字符串或数组）
        let fields = if let Some(fields_value) = raw.fields {
            match fields_value {
                serde_json::Value::String(fields_str) => {
                    // 逗号分隔的字符串
                    fields_str.split(',').map(|s| s.trim().to_string()).collect()
                }
                serde_json::Value::Array(fields_arr) => {
                    // 数组格式
                    fields_arr
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                }
                _ => Vec::new(),
            }
        } else {
            Vec::new()
        };

        Ok(ResponseData {
            fields,
            items: raw.items.unwrap_or_default(),
        })
    }
}

impl TushareResponse {
    /// 检查响应是否成功
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    /// 将二维数组转换为对象数组
    pub fn to_objects(&self) -> Vec<HashMap<String, serde_json::Value>> {
        let mut result = Vec::new();

        if let Some(ref data) = self.data {
            for item in &data.items {
                let mut map = HashMap::new();
                for (i, field) in data.fields.iter().enumerate() {
                    if i < item.len() {
                        map.insert(field.clone(), item[i].clone());
                    }
                }
                result.push(map);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_response_success() {
        let resp = TushareResponse {
            request_id: "123".to_string(),
            code: 0,
            msg: "success".to_string(),
            data: None,
        };
        assert!(resp.is_success());
    }

    #[test]
    fn test_response_error() {
        let resp = TushareResponse {
            request_id: "123".to_string(),
            code: -1,
            msg: "error".to_string(),
            data: None,
        };
        assert!(!resp.is_success());
    }

    #[test]
    fn test_to_objects() {
        let json_data = json!({
            "fields": "ts_code,name",
            "items": [["000001.SZ", "平安银行"], ["000002.SZ", "万科A"]]
        });

        let data: ResponseData = serde_json::from_value(json_data).unwrap();
        let resp = TushareResponse {
            request_id: "123".to_string(),
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        };

        let objects = resp.to_objects();
        assert_eq!(objects.len(), 2);
        assert_eq!(objects[0].get("ts_code").unwrap(), "000001.SZ");
        assert_eq!(objects[0].get("name").unwrap(), "平安银行");
    }
}
