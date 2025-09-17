use crate::models::about::About;
use crate::rbatis::RBATIS;
use rbs::to_value;
use rbs::value::map::ValueMap;

pub struct AboutService;

impl AboutService {
    ///获取所有about信息
    pub(crate) async fn get_about() -> ValueMap {
        let mut map = ValueMap::new();
        About::select_all(&RBATIS.acquire().await.expect("异常"))
            .await
            .unwrap_or_else(|e| {
                //出现异常情况
                log::error!("get_about error：{}", e);
                vec![]
            })
            .iter()
            .for_each(|item: &About| {
                //转HTML
                if &item.name_en == "content" {
                    let content = markdown::to_html(&item.value);
                    map.insert(to_value!(&item.name_en), to_value!(content));
                } else {
                    map.insert(to_value!(&item.name_en), to_value!(&item.value));
                }
            });
        map
    }
}
