//! API 定义结构

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API 定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDefinition {
    /// API 名称
    pub name: String,
    /// API 描述
    pub description: String,
    /// API 类别
    pub category: String,
    /// 文档 ID
    #[serde(rename = "docId")]
    pub doc_id: i32,
    /// 输入参数
    pub parameters: Vec<ApiParameter>,
    /// 输出字段
    #[serde(rename = "outputFields")]
    pub output_fields: Vec<ApiOutputField>,
    /// 所需积分
    #[serde(rename = "requiresPoints", default)]
    pub requires_points: Option<i32>,
}

/// API 参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParameter {
    /// 参数名
    pub name: String,
    /// 参数类型
    #[serde(rename = "type")]
    pub param_type: String,
    /// 是否必选
    pub required: bool,
    /// 参数描述
    pub description: String,
}

/// API 输出字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOutputField {
    /// 字段名
    pub name: String,
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: String,
    /// 是否默认显示
    #[serde(rename = "defaultShow", default)]
    pub default_show: bool,
    /// 字段描述
    pub description: String,
}

/// API 注册表类型
type ApiRegistry = HashMap<String, Vec<ApiDefinition>>;

/// 全局 API 定义缓存
static mut API_DEFINITIONS_CACHE: Option<HashMap<String, ApiDefinition>> = None;

/// 加载 API 定义
///
/// 从嵌入的 JSON 文件加载 API 定义
pub fn load_api_definitions() -> HashMap<String, ApiDefinition> {
    unsafe {
        // 如果已经加载过，直接返回缓存
        if let Some(ref cache) = API_DEFINITIONS_CACHE {
            return cache.clone();
        }
    }

    // 从 JSON 文件加载
    let json_data = include_str!("definitions.json");
    let all_definitions: HashMap<String, ApiDefinition> =
        serde_json::from_str(json_data).unwrap_or_else(|e| {
            eprintln!("警告: 无法加载 API 定义: {}", e);
            HashMap::new()
        });

    // 过滤掉非英文命名的接口（只保留有效的 API）
    let definitions: HashMap<String, ApiDefinition> = all_definitions
        .into_iter()
        .filter(|(name, def)| {
            // 只保留英文名称的接口（包含 a-z、0-9、_、-）
            // 过滤掉中文命名的条目，如 "复权行情"、"数据索引" 等
            is_valid_api_name(name) &&
            // 确保有输出字段（有效的 API 应该有输出字段）
            (!def.output_fields.is_empty() || !def.parameters.is_empty())
        })
        .collect();

    unsafe {
        // 缓存结果
        API_DEFINITIONS_CACHE = Some(definitions.clone());
    }

    definitions
}

/// 检查是否是有效的 API 名称
/// 有效名称只包含英文、数字、下划线和连字符
fn is_valid_api_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // 检查是否只包含 ASCII 字符（排除中文）
    name.chars().all(|c| c.is_ascii() && (c.is_alphanumeric() || c == '_' || c == '-'))
}

/// 获取所有 API 类别
pub fn get_categories() -> Vec<&'static str> {
    vec![
        "股票数据",
        "宏观经济",
        "指数专题",
        "债券专题",
        "期货数据",
        "港股数据",
        "美股数据",
        "ETF专题",
        "公募基金",
        "行业经济",
        "大模型语料专题数据",
        "期权数据",
        "其他",
        "财富管理",
        "外汇数据",
        "现货数据",
        "基金数据",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_definition_serialization() {
        let json = r#"{
            "name": "stock_basic",
            "description": "股票列表",
            "category": "股票数据",
            "docId": 25,
            "parameters": [],
            "outputFields": [],
            "requiresPoints": 2000
        }"#;

        let def: ApiDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(def.name, "stock_basic");
        assert_eq!(def.description, "股票列表");
        assert_eq!(def.doc_id, 25);
        assert_eq!(def.requires_points, Some(2000));
    }

    #[test]
    fn test_get_categories() {
        let categories = get_categories();
        assert_eq!(categories.len(), 17);
        assert!(categories.contains(&"股票数据"));
    }
}
