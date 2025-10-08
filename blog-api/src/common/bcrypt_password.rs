use bcrypt::{hash, verify, DEFAULT_COST};

use crate::error::WebError;

pub struct UserBcrypt;

impl UserBcrypt {
    /**
     * 验证密码
     */
    pub fn verify_password(hashed: &str, password: &str) -> Result<bool, WebError> {
        match verify(password, hashed) {
            Ok(result) => Ok(result),
            Err(err) => Err(WebError::Jwt(err.to_string())),
        }
    }

    pub fn hash_password(password: &str) -> Result<String, WebError> {
        // 使用默认成本因子（12）进行哈希
        match hash(password, DEFAULT_COST) {
            Ok(result) => Ok(result),
            Err(err) => Err(WebError::Jwt(err.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_verify_password() {
        let result = UserBcrypt::verify_password(
            "$2b$12$J9Z8AKjIMluR34q7ksRUBe2k5glay6rVndjIzCzeN94Gt.o5BTRXW",
            "123456",
        )
        .unwrap_or_default();
        println!(
            "测试加密的字符串是:{:?}  原始密码是123456 解密结果是{:?}",
            "$2b$12$J9Z8AKjIMluR34q7ksRUBe2k5glay6rVndjIzCzeN94Gt.o5BTRXW", result
        );
        assert_eq!(result, true)
    }

    #[test]
    fn test_hash_password() {
        let result = UserBcrypt::hash_password("123456").unwrap_or_default();
        println!("测试加密后的字符串是:{:?}", result);
        assert_eq!(!result.is_empty(), true)
    }
}
