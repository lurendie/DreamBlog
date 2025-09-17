use std::collections::HashMap;

use actix_web::{get, web::Query, HttpResponse, Responder};
use rbatis::IPageRequest;
use rbs::to_value;
use rbs::value::map::ValueMap;
use crate::service::MomentService;

use crate::models::vo::result::Result;



//动态
#[get("/moments")]
pub(crate) async fn moments(params: Query<HashMap<String, String>>) -> impl Responder{
    let page :usize;
    if params.get("pageNum") !=None{
        page= params.get("pageNum").expect("转换失败").parse::<u32>().expect("转换失败")as usize;
    }else {
        page=1;
    }
    let page =MomentService::get_moments(page).await;
    let mut data:ValueMap=ValueMap::new();
    data.insert(to_value!("list"), to_value!(&page.records));
    data.insert(to_value!("totalPage"), to_value!(&page.pages()));
    let result = Result::ok("请求成功".to_string(), Some(data));
    HttpResponse::Ok().json(result)
}