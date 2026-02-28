//! API 搜索功能

use crate::api::definitions::ApiDefinition;
use std::collections::HashMap;

/// 根据名称查找 API
pub fn find_api_by_name<'a>(
    definitions: &'a HashMap<String, ApiDefinition>,
    name: &str,
) -> Option<&'a ApiDefinition> {
    definitions.get(name)
}

/// 搜索 API
///
/// 在 API 名称和描述中搜索关键词
pub fn search_apis<'a>(
    definitions: &'a HashMap<String, ApiDefinition>,
    keyword: &str,
) -> Vec<&'a ApiDefinition> {
    let keyword_lower = keyword.to_lowercase();

    definitions
        .values()
        .filter(|api| {
            api.name.to_lowercase().contains(&keyword_lower)
                || api.description.to_lowercase().contains(&keyword_lower)
        })
        .collect()
}

/// 按类别获取 API
pub fn get_apis_by_category<'a>(
    definitions: &'a HashMap<String, ApiDefinition>,
    category: &str,
) -> Vec<&'a ApiDefinition> {
    definitions
        .values()
        .filter(|api| api.category == category)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::definitions::ApiDefinition;

    #[test]
    fn test_find_api_by_name() {
        let mut definitions = HashMap::new();

        definitions.insert(
            "stock_basic".to_string(),
            ApiDefinition {
                name: "stock_basic".to_string(),
                description: "股票列表".to_string(),
                category: "股票数据".to_string(),
                doc_id: 25,
                parameters: vec![],
                output_fields: vec![],
                requires_points: Some(2000),
            },
        );

        let api = find_api_by_name(&definitions, "stock_basic");
        assert!(api.is_some());
        assert_eq!(api.unwrap().name, "stock_basic");

        let api = find_api_by_name(&definitions, "nonexistent");
        assert!(api.is_none());
    }

    #[test]
    fn test_search_apis() {
        let mut definitions = HashMap::new();

        definitions.insert(
            "stock_basic".to_string(),
            ApiDefinition {
                name: "stock_basic".to_string(),
                description: "获取股票列表".to_string(),
                category: "股票数据".to_string(),
                doc_id: 25,
                parameters: vec![],
                output_fields: vec![],
                requires_points: Some(2000),
            },
        );

        definitions.insert(
            "daily".to_string(),
            ApiDefinition {
                name: "daily".to_string(),
                description: "股票日线行情".to_string(),
                category: "股票数据".to_string(),
                doc_id: 30,
                parameters: vec![],
                output_fields: vec![],
                requires_points: Some(2000),
            },
        );

        // 搜索 "股票"
        let results = search_apis(&definitions, "股票");
        assert_eq!(results.len(), 2);

        // 搜索 "daily"
        let results = search_apis(&definitions, "daily");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "daily");
    }

    #[test]
    fn test_get_apis_by_category() {
        let mut definitions = HashMap::new();

        definitions.insert(
            "stock_basic".to_string(),
            ApiDefinition {
                name: "stock_basic".to_string(),
                description: "股票列表".to_string(),
                category: "股票数据".to_string(),
                doc_id: 25,
                parameters: vec![],
                output_fields: vec![],
                requires_points: Some(2000),
            },
        );

        definitions.insert(
            "index_basic".to_string(),
            ApiDefinition {
                name: "index_basic".to_string(),
                description: "指数列表".to_string(),
                category: "指数数据".to_string(),
                doc_id: 26,
                parameters: vec![],
                output_fields: vec![],
                requires_points: None,
            },
        );

        let results = get_apis_by_category(&definitions, "股票数据");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "stock_basic");

        let results = get_apis_by_category(&definitions, "指数数据");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "index_basic");
    }
}
