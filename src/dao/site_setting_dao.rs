use crate::models::site_setting::SiteSetting;
use crate::rbatis::RBATIS;

    pub async fn get_list() -> Vec<SiteSetting> {
        let site_settings = SiteSetting::select_all(&RBATIS.acquire().await.expect("异常")).await;
        let v =match site_settings {
            Ok(v)=>v,
            Err(..)=>vec!(),
        };
        v
    }
