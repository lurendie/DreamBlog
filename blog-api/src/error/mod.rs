/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:14:37
 */
pub mod web_error;
mod data_base_error;
pub mod error_code;
pub use web_error::WebError;
pub use data_base_error::DataBaseError;
pub use error_code::WebErrorCode;
