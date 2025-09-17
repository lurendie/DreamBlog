//随机博客显示5条
pub(crate) const RANDOM_BLOG_LIMIT_NUM: usize = 5;
//最新推荐博客显示3条
pub(crate) const NEW_BLOG_PAGE_SIZE: usize = 3;
//每页显示5条博客简介
pub(crate) const PAGE_SIZE: u64 = 5;
pub const _ORDER_BY: &str = "is_top desc, create_time desc";

pub(crate) const _PRIVATE_BLOG_DESCRIPTION: &str = "此文章受密码保护！";
