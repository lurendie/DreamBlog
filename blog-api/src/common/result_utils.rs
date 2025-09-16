/*
 * @Author: lurendie
 * @Date: 2024-05-16 19:14:37
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-16 19:14:37
 */
/// Result 类型扩展工具
pub trait ResultUtils<T, E> {
    /// 成功时执行操作，失败时执行另一个操作
    fn unwrap_or_else<F>(self, op: F) -> T 
    where
        F: FnOnce(E) -> T;

    /// 将错误转换为自定义错误类型
    fn map_error<F, E2>(self, op: F) -> Result<T, E2>
    where
        F: FnOnce(E) -> E2;
}

impl<T, E> ResultUtils<T, E> for Result<T, E> {
    fn unwrap_or_else<F>(self, op: F) -> T 
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Ok(value) => value,
            Err(err) => op(err),
        }
    }

    fn map_error<F, E2>(self, op: F) -> Result<T, E2>
    where
        F: FnOnce(E) -> E2,
    {
        self.map_err(op)
    }
}
