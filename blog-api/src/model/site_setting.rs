use crate::entity::site_setting::Model;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SiteSetting {
    //#[crud_table(column: "id")]
    pub id: Option<i64>,
      #[serde(rename = "nameEn")]
    pub name_en: String,
      #[serde(rename = "nameZh")]
    pub name_zh: String,
    //#[crud_table(column: "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: i32, //1基础设置，2页脚徽标，3资料卡，4友链信息
}

// 实现从 Model 类型到 SiteSetting 类型的转换
impl From<Model> for SiteSetting {
    // 定义转换函数，将 Model 类型的值转换为 SiteSetting 类型的值
    fn from(value: Model) -> Self {
        // 创建并返回一个新的 SiteSetting 实例
        SiteSetting {
            // 将 Model 的 id 字段转换为 SiteSetting 的 id 字段，使用 Some 包装
            id: Some(value.id),
            // 将 Model 的 name_en 字段转换为 SiteSetting 的 name_en 字段，如果为 None 则使用默认值
            name_en: value.name_en.unwrap_or_default(),
            // 将 Model 的 name_zh 字段转换为 SiteSetting 的 name_zh 字段，如果为 None 则使用默认值
            name_zh: value.name_zh.unwrap_or_default(),
            // 将 Model 的 value 字段转换为 SiteSetting 的 value 字段，如果为 None 则使用默认值
            value: value.value.unwrap_or_default(),
            // 将 Model 的 r#type 字段转换为 SiteSetting 的 r#type 字段，如果为 None 则使用默认值
            // 注意：r# 是 Rust 中的原始标识符(raw identifier)，用于使用保留字作为标识符
            r#type: value.r#type.unwrap_or_default(),
        }
    }
}
