// use actix_jwt_session::Extractors;
// use actix_jwt_session::SessionMiddlewareFactory;
// use actix_jwt_session::SessionStorage;
// use actix_jwt_session::JWT_HEADER_NAME;
use serde::Deserialize;

use serde::Serialize;

//use crate::redis_client::REDIS_CL_IENT;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    Web,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct AppClaims {
    #[serde(rename = "exp")]
    pub expiration_time: u64,
    #[serde(rename = "iat")]
    pub issues_at: usize,
    // Account login
    #[serde(rename = "username")]
    pub subject: String,
    // #[serde(rename = "aud")]
    // pub audience: Audience,
    #[serde(rename = "jti")]
    pub jwt_id: actix_jwt_session::Uuid,
    #[serde(rename = "aci")]
    pub account_id: i32,
    #[serde(rename = "nbf")]
    pub not_before: u64,
}

impl actix_jwt_session::Claims for AppClaims {
    fn jti(&self) -> actix_jwt_session::Uuid {
        self.jwt_id
    }

    fn subject(&self) -> &str {
        &self.subject
    }
}

// pub struct JWT;
// impl JWT {
//     pub fn create<T: actix_jwt_session::Claims>() -> (SessionStorage, SessionMiddlewareFactory<T>) {
//         // create new [SessionStorage] and [SessionMiddlewareFactory]
//         let (storage, factory) = SessionMiddlewareFactory::<T>::build_ed_dsa()
//             // pass redis connection
//             .with_redis_pool(REDIS_CL_IENT.clone())
//             .with_extractors(Extractors::default().with_jwt_header(JWT_HEADER_NAME))
//             // Check if header "Authorization" exists and contains Bearer with encoded JWT
//             // Check if cookie "jwt" exists and contains encoded JWT
//             //.with_jwt_cookie(JWT_COOKIE_NAME)
//             //.with_refresh_header(REFRESH_HEADER_NAME)
//             // Check if cookie "jwt" exists and contains encoded JWT
//             //.with_refresh_cookie(REFRESH_COOKIE_NAME)
//             .finish();
//         (storage, factory)
//     }
// }

#[cfg(test)]
mod test {

    #[test]
    fn test() {}
}
