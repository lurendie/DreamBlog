use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::Arc;
use std::vec;

use actix_jwt_session::Algorithm;
use actix_jwt_session::Authenticated;
use actix_jwt_session::Claims;
use actix_jwt_session::Error;
use actix_jwt_session::ExtractorKind;
use actix_jwt_session::Extractors;
use actix_jwt_session::SessionExtractor;
use actix_jwt_session::SessionStorage;
use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use async_trait::async_trait;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use serde::Deserialize;
use serde::Serialize;

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
pub struct CustomExtractor;

impl CustomExtractor {
    pub fn new(name: &'static str) -> Extractors<AppClaims> {
        let e: Extractors<AppClaims> =
            Extractors::new(vec![Arc::new(CustomHeaderExtractor::new(name))], vec![]);
        e
    }
}
#[derive(Debug)]
struct CustomHeaderExtractor<ClaimsType> {
    __ty: PhantomData<ClaimsType>,
    header_name: &'static str,
}

impl<ClaimsType: Claims> CustomHeaderExtractor<ClaimsType> {
    /// Creates new header extractor.
    /// It will extract token data from header with given name
    pub fn new(header_name: &'static str) -> Self {
        Self {
            __ty: Default::default(),
            header_name,
        }
    }
}

#[async_trait(?Send)]
impl<ClaimsType: Claims> SessionExtractor<ClaimsType> for CustomHeaderExtractor<ClaimsType> {
    async fn extract_token_text<'req>(
        &self,
        req: &'req mut ServiceRequest,
    ) -> Option<Cow<'req, str>> {
        req.headers()
            .get(self.header_name)
            .and_then(|h| h.to_str().ok())
            .map(|h| h.to_owned().into())
    }
    fn extractor_key(&self) -> Option<(ExtractorKind, Cow<'static, str>)> {
        Some((ExtractorKind::Header, self.header_name.into()))
    }

    async fn extract_claims(
        &self,
        req: &mut ServiceRequest,
        jwt_encoding_key: Arc<EncodingKey>,
        jwt_decoding_key: Arc<DecodingKey>,
        algorithm: Algorithm,
        storage: SessionStorage,
    ) -> Result<(), Error> {
        let Some(as_str) = self.extract_token_text(req).await else {
            return Ok(());
        };
        let decoded_claims = self.decode(&as_str, jwt_decoding_key, algorithm)?;
        self.validate(&decoded_claims, storage).await?;
        req.extensions_mut().insert(Authenticated {
            claims: Arc::new(decoded_claims),
            jwt_encoding_key,
            algorithm,
        });
        Ok(())
    }
}
