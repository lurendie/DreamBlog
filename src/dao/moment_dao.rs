use crate::models::moment::Moment;
use crate::rbatis::RBATIS;
use rbatis::Error;
use rbatis::Page;
use rbatis::PageRequest;
pub struct MomentDao;
impl MomentDao {
    //获取公开的动态
    pub(crate) async fn get_moments(page_num: usize) -> Result<Page<Moment>, Error> {
        let moments = Moment::select_page(
            &RBATIS.acquire().await.unwrap(),
            &PageRequest::new(page_num as u64, 5),
        )
        .await;
        moments
    }
    
}
