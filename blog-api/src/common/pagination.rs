/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:14:37
 */
use serde::{Deserialize, Serialize};

/// 分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// 页码，从1开始
    pub page: u64,
    /// 每页大小
    pub page_size: u64,
}

impl PaginationParams {
    /// 创建新的分页参数
    pub fn new(page: u64, page_size: u64) -> Self {
        Self {
            page: page.max(1),
            page_size: page_size.max(1).min(100), // 限制最大每页100条
        }
    }

    /// 获取偏移量
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.page_size
    }
}

/// 分页结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination<T> {
    /// 当前页码
    pub page: u64,
    /// 每页大小
    pub page_size: u64,
    /// 总记录数
    pub total: u64,
    /// 总页数
    pub total_pages: u64,
    /// 数据列表
    pub items: Vec<T>,
}

impl<T> Pagination<T> {
    /// 创建新的分页结果
    pub fn new(items: Vec<T>, page: u64, page_size: u64, total: u64) -> Self {
        let total_pages = if total == 0 {
            0
        } else {
            ((total - 1) / page_size) + 1
        };

        Self {
            page,
            page_size,
            total,
            total_pages,
            items,
        }
    }

    /// 是否有上一页
    pub fn has_prev(&self) -> bool {
        self.page > 1
    }

    /// 是否有下一页
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    /// 上一页页码
    pub fn prev_page(&self) -> Option<u64> {
        if self.has_prev() {
            Some(self.page - 1)
        } else {
            None
        }
    }

    /// 下一页页码
    pub fn next_page(&self) -> Option<u64> {
        if self.has_next() {
            Some(self.page + 1)
        } else {
            None
        }
    }
}
