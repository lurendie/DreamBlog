/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:14:37
 */
use crate::{
    error::WebError,
    model::{SearchParams, SearchRequest},
};
use actix_web::web::Query;
use std::collections::HashMap;

/// 参数处理工具
pub struct ParamUtils;

impl ParamUtils {
    /// 从Query中提取参数
    pub fn _extract_query_params<T>(query: Query<T>) -> T
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        query.into_inner()
    }

    /// 从HashMap中获取字符串参数
    pub fn get_string_param(
        params: &HashMap<String, String>,
        key: &str,
    ) -> Result<String, WebError> {
        params
            .get(key)
            .cloned()
            .ok_or_else(|| WebError::Validation(format!("缺少必要参数: {}", key)))
    }

    /// 从HashMap中获取整数参数
    pub fn get_i64_param(params: &HashMap<String, String>, key: &str) -> Result<i64, WebError> {
        let value = Self::get_string_param(params, key)?;
        value
            .parse()
            .map_err(|_| WebError::Validation(format!("参数 {} 不是有效的整数", key)))
    }

    /// 从HashMap中获取正整数参数
    pub fn _get_positive_i64_param(
        params: &HashMap<String, String>,
        key: &str,
    ) -> Result<i64, WebError> {
        let value = Self::get_i64_param(params, key)?;
        if value <= 0 {
            return Err(WebError::Validation(format!("参数 {} 必须是正整数", key)));
        }
        Ok(value)
    }

    /// 从HashMap中获取布尔参数
    pub fn get_bool_param(params: &HashMap<String, String>, key: &str) -> Result<bool, WebError> {
        let value = Self::get_string_param(params, key)?;
        match value.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" => Ok(false),
            _ => Err(WebError::Validation(format!(
                "参数 {} 不是有效的布尔值",
                key
            ))),
        }
    }

    /// 检查参数是否存在
    pub fn _check_param_exists(
        params: &HashMap<String, String>,
        key: &str,
    ) -> Result<(), WebError> {
        if !params.contains_key(key) {
            return Err(WebError::Validation(format!("缺少必要参数: {}", key)));
        }
        Ok(())
    }

    /// 验证分页参数
    pub fn validate_pagination_params(
        params: &HashMap<String, String>,
    ) -> Result<(u64, u64), WebError> {
        let page = match params.get("pageNum") {
            Some(page_str) => page_str
                .parse()
                .map_err(|_| WebError::Validation("页码必须是有效的整数".to_string()))?,
            None => 1, // 默认第一页
        };

        let page_size = match params.get("pageSize") {
            Some(size_str) => {
                let size = size_str
                    .parse()
                    .map_err(|_| WebError::Validation("每页大小必须是有效的整数".to_string()))?;
                if size == 0 || size > 100 {
                    return Err(WebError::Validation("每页大小必须在1-100之间".to_string()));
                }
                size
            }
            None => 10, // 默认每页10条
        };

        Ok((page.max(1), page_size.max(1)))
    }

    /**
     * 验证搜索请求参数 返回SearchParams 此结构体经过验证
     */
    pub async fn validate_request_params(param: &SearchRequest) -> Result<SearchParams, WebError> {
        let mut search_params = SearchParams::new();
        if let Some(page_num) = param.get_page_num() {
            if matches!(page_num, 0) {
                return Err(WebError::Validation("页码不能为0".to_string()));
            }
            search_params.page_num = page_num;
        } else {
            search_params.page_num = 1;
        }
        if let Some(page_size) = param.get_page_size() {
            if matches!(page_size, 0) {
                return Err(WebError::Validation("每页大小不能为0".to_string()));
            }
            search_params.page_size = page_size;
        }
        if let Some(blog_id) = param.get_blog_id() {
            if matches!(blog_id, 0) {
                return Err(WebError::Validation("博客ID不能为0".to_string()));
            }
            search_params.blog_id = blog_id;
        }
        if let Some(category_id) = param.get_category_id() {
            if matches!(category_id, 0) {
                return Err(WebError::Validation("分类ID不能为0".to_string()));
            }
            search_params.category_id = category_id;
        }
        if let Some(password) = param.get_password() {
            if password.is_empty() {
                return Err(WebError::Validation("密码错误".to_string()));
            }
            search_params.password = password;
        }
        if let Some(page_type) = param.get_page() {
            if page_type >= 3 {
                return Err(WebError::Validation("页面类型错误".to_string()));
            }
            search_params.page = page_type;
        }
        if let Some(title) = param.get_title() {
            if title.contains("") || title.is_empty() {
                return Err(WebError::Validation("标题不能为空".to_string()));
            }
            search_params.title = title;
        }
        Ok(search_params)
    }
}
