use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
#[derive(Debug, Clone, Serialize)]
pub enum TypeValue {
    Int32(i32),
    String(String),
}

impl<'de> Deserialize<'de> for TypeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 定义一个 Visitor 来处理反序列化
        struct TypeValueVisitor;

        impl<'de> Visitor<'de> for TypeValueVisitor {
            type Value = TypeValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an integer or a string")
            }
            // 处理 i64 类型的值
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(TypeValue::Int32(value as i32))
            }

            // 处理 u64 类型的值
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(TypeValue::Int32(value as i32))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(TypeValue::String(v))
            }

            // 处理字符串类型的值
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(TypeValue::String(value.to_string()))
            }
        }

        deserializer.deserialize_any(TypeValueVisitor)
    }
}
