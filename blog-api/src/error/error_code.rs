/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:14:37
 */
/// 错误码常量定义
pub struct ErrorCode;

impl ErrorCode {
    /// 成功
    pub const SUCCESS: u16 = 200;

    /// 参数验证错误
    pub const VALIDATION_ERROR: u16 = 400;

    /// 未授权
    pub const UNAUTHORIZED: u16 = 401;

    /// 禁止访问
    pub const FORBIDDEN: u16 = 403;

    /// 资源未找到
    pub const NOT_FOUND: u16 = 404;

    /// 内部服务器错误
    pub const INTERNAL_ERROR: u16 = 500;

    /// 业务逻辑错误
    pub const BUSINESS_ERROR: u16 = 501;

    /// JWT错误
    pub const JWT_ERROR: u16 = 502;

    /// Redis错误
    pub const REDIS_ERROR: u16 = 503;

    /// 数据库错误
    pub const DATABASE_ERROR: u16 = 504;

    /// 自定义错误
    pub const CUSTOM_ERROR: u16 = 600;
}
