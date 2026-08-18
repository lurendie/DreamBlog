use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MomentDTO {
    #[serde(default)]
    pub(crate) id: Option<i64>,
    pub(crate) content: String,
    #[serde(
        default,
        rename(deserialize = "createTime"),
        deserialize_with = "de_naive_datetime_opt"
    )]
    pub(crate) create_time: Option<NaiveDateTime>,
    #[serde(default, deserialize_with = "de_i32_opt")]
    pub(crate) likes: Option<i32>,
    #[serde(rename(deserialize = "published"))]
    pub(crate) is_published: bool,
}

/// 宽松解析创建时间：接受 null、空串、"YYYY-MM-DD HH:mm:ss"、
/// "YYYY-MM-DDTHH:mm:ss[.fff]" 以及带 Z 的 RFC3339（前端 Date 序列化产物）
fn de_naive_datetime_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    let Some(s) = s else { return Ok(None); };
    let s = s.trim();
    if s.is_empty() {
        return Ok(None);
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(Some(dt.with_timezone(&Local).naive_local()));
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f") {
        return Ok(Some(dt));
    }
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f")
        .map(Some)
        .map_err(serde::de::Error::custom)
}

/// 宽松解析点赞数：接受 null、数字、数字字符串；空字符串视为 None
fn de_i32_opt<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Repr {
        Num(i32),
        Str(String),
    }
    match Option::<Repr>::deserialize(deserializer)? {
        None => Ok(None),
        Some(Repr::Num(n)) => Ok(Some(n)),
        Some(Repr::Str(s)) if s.trim().is_empty() => Ok(None),
        Some(Repr::Str(s)) => s
            .trim()
            .parse()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_time_accepts_null_and_empty() {
        let dto: MomentDTO =
            serde_json::from_str(r#"{"content":"x","createTime":null,"published":true}"#)
                .unwrap();
        assert!(dto.create_time.is_none());
        let dto: MomentDTO =
            serde_json::from_str(r#"{"content":"x","createTime":"","published":true}"#)
                .unwrap();
        assert!(dto.create_time.is_none());
    }

    #[test]
    fn create_time_accepts_space_iso_and_rfc3339() {
        let dto: MomentDTO = serde_json::from_str(
            r#"{"content":"x","createTime":"2026-08-17 10:00:00","published":true}"#,
        )
        .unwrap();
        assert_eq!(
            dto.create_time,
            Some(
                NaiveDateTime::parse_from_str("2026-08-17 10:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
            )
        );
        let dto: MomentDTO = serde_json::from_str(
            r#"{"content":"x","createTime":"2026-08-17T10:00:00.000Z","published":true}"#,
        )
        .unwrap();
        assert!(dto.create_time.is_some());
    }

    #[test]
    fn create_time_rejects_garbage() {
        let result = serde_json::from_str::<MomentDTO>(
            r#"{"content":"x","createTime":"not-a-date","published":true}"#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn likes_accepts_null_number_and_strings() {
        let dto: MomentDTO =
            serde_json::from_str(r#"{"content":"x","likes":null,"published":true}"#).unwrap();
        assert_eq!(dto.likes, None);
        let dto: MomentDTO =
            serde_json::from_str(r#"{"content":"x","likes":5,"published":true}"#).unwrap();
        assert_eq!(dto.likes, Some(5));
        let dto: MomentDTO =
            serde_json::from_str(r#"{"content":"x","likes":"","published":true}"#).unwrap();
        assert_eq!(dto.likes, None);
        let dto: MomentDTO =
            serde_json::from_str(r#"{"content":"x","likes":"7","published":true}"#).unwrap();
        assert_eq!(dto.likes, Some(7));
    }
}

// impl MomentDTO {
//     pub fn new(
//         content: Option<String>,
//         create_time: Option<String>,
//         likes: Option<u64>,
//         is_published: bool,
//     ) -> Self {
//         Self {
//             id: None,
//             content,
//             create_time,
//             likes,
//             is_published,
//         }
//     }

//     // pub fn set_id(&mut self, id: u16) {
//     //     self.id = Some(id);
//     // }
//     // pub fn get_id(&self) -> Option<u16> {
//     //     self.id
//     // }

//     // pub fn get_content(&self) -> Option<String> {
//     //     self.content.clone()
//     // }

//     // pub fn set_content(&mut self, content: String) {
//     //     self.content = Some(content);
//     // }

//     // pub fn get_create_time(&self) -> Option<String> {
//     //     self.create_time.clone()
//     // }

//     // pub fn set_create_time(&mut self, create_time: String) {
//     //     self.create_time = Some(create_time);
//     // }

//     // pub fn get_likes(&self) -> u64 {
//     //     self.likes.unwrap_or_default()
//     // }

//     // pub fn set_likes(&mut self, likes: u64) {
//     //     self.likes = Some(likes);
//     // }

//     // pub fn get_is_published(&self) -> bool {
//     //     self.is_published
//     // }

//     // pub fn set_is_published(&mut self, is_published: bool) {
//     //     self.is_published = is_published;
//     // }
// }
