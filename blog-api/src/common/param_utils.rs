/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:14:37
 */
use actix_web::web::Query;
use crate::error::AppError;
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
    pub fn get_string_param(params: &HashMap<String, String>, key: &str) -> Result<String, AppError> {
        params.get(key)
            .cloned()
            .ok_or_else(|| AppError::Validation(format!("缺少必要参数: {}", key)))
    }

    /// 从HashMap中获取整数参数
    pub fn get_i64_param(params: &HashMap<String, String>, key: &str) -> Result<i64, AppError> {
        let value = Self::get_string_param(params, key)?;
        value.parse()
            .map_err(|_| AppError::Validation(format!("参数 {} 不是有效的整数", key)))
    }

    /// 从HashMap中获取正整数参数
    pub fn _get_positive_i64_param(params: &HashMap<String, String>, key: &str) -> Result<i64, AppError> {
        let value = Self::get_i64_param(params, key)?;
        if value <= 0 {
            return Err(AppError::Validation(format!("参数 {} 必须是正整数", key)));
        }
        Ok(value)
    }

    /// 从HashMap中获取布尔参数
    pub fn _get_bool_param(params: &HashMap<String, String>, key: &str) -> Result<bool, AppError> {
        let value = Self::get_string_param(params, key)?;
        match value.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" => Ok(false),
            _ => Err(AppError::Validation(format!("参数 {} 不是有效的布尔值", key))),
        }
    }

    /// 检查参数是否存在
    pub fn _check_param_exists(params: &HashMap<String, String>, key: &str) -> Result<(), AppError> {
        if !params.contains_key(key) {
            return Err(AppError::Validation(format!("缺少必要参数: {}", key)));
        }
        Ok(())
    }

    /// 验证分页参数
    pub fn validate_pagination_params(params: &HashMap<String, String>) -> Result<(u64, u64), AppError> {
        let page = match params.get("pageNum") {
            Some(page_str) => page_str.parse()
                .map_err(|_| AppError::Validation("页码必须是有效的整数".to_string()))?,
            None => 1, // 默认第一页
        };

        let page_size = match params.get("pageSize") {
            Some(size_str) => {
                let size = size_str.parse()
                    .map_err(|_| AppError::Validation("每页大小必须是有效的整数".to_string()))?;
                if size == 0 || size > 100 {
                    return Err(AppError::Validation("每页大小必须在1-100之间".to_string()));
                }
                size
            },
            None => 10, // 默认每页10条
        };

        Ok((page.max(1), page_size.max(1)))
    }
}
