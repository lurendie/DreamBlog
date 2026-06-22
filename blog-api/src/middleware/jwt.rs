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
use actix_jwt_session::JwtSigningKeys;
use actix_jwt_session::SessionExtractor;
use actix_jwt_session::SessionMiddlewareFactory;
use actix_jwt_session::SessionStorage;
use actix_jwt_session::JWT_HEADER_NAME;
use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use async_trait::async_trait;
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Validation;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub struct AppClaims {
    #[serde(rename = "exp")]
    pub expiration_time: u64,
    #[serde(rename = "iat")]
    pub issues_at: usize,
    #[serde(rename = "username")]
    pub subject: String,
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

pub fn build_session_storage() -> (SessionStorage, SessionMiddlewareFactory<AppClaims>) {
    let keys = JwtSigningKeys::load_or_create();
    let encoding_key = Arc::new(keys.encoding_key);
    let decoding_key = Arc::new(keys.decoding_key);
    let storage = SessionStorage::new(
        Arc::new(NoopTokenStorage),
        encoding_key.clone(),
        Algorithm::EdDSA,
    );
    let factory = SessionMiddlewareFactory::build(encoding_key, decoding_key, Algorithm::EdDSA)
        .with_storage(storage.clone())
        .with_extractors(CustomExtractor::new(JWT_HEADER_NAME))
        .finish()
        .1;
    (storage, factory)
}

#[derive(Clone)]
struct NoopTokenStorage;

#[async_trait(?Send)]
impl actix_jwt_session::TokenStorage for NoopTokenStorage {
    async fn get_by_jti(self: Arc<Self>, _jti: &[u8]) -> Result<Vec<u8>, actix_jwt_session::Error> {
        Err(actix_jwt_session::Error::NotFound)
    }

    async fn set_by_jti(
        self: Arc<Self>,
        _jwt_jti: &[u8],
        _refresh_jti: &[u8],
        _bytes: &[u8],
        _exp: actix_jwt_session::Duration,
    ) -> Result<(), actix_jwt_session::Error> {
        Ok(())
    }

    async fn remove_by_jti(self: Arc<Self>, _jti: &[u8]) -> Result<(), actix_jwt_session::Error> {
        Ok(())
    }
}

pub struct CustomExtractor;

impl CustomExtractor {
    pub fn new(name: &'static str) -> Extractors<AppClaims> {
        Extractors::new(vec![Arc::new(CustomHeaderExtractor::new(name))], vec![])
    }
}

#[derive(Debug)]
struct CustomHeaderExtractor<ClaimsType> {
    __ty: PhantomData<ClaimsType>,
    header_name: &'static str,
}

impl<ClaimsType: Claims> CustomHeaderExtractor<ClaimsType> {
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
        if self.validate_login_path(req.path()).await {
            return Ok(());
        }
        let Some(as_str) = self.extract_token_text(req).await else {
            return Ok(());
        };
        let decoded_claims = match self.decode(&as_str, jwt_decoding_key, algorithm) {
            Ok(claims) => claims,
            Err(_) => return Ok(()),
        };
        let _ = storage;
        req.extensions_mut().insert(Authenticated {
            claims: Arc::new(decoded_claims),
            jwt_encoding_key,
            algorithm,
        });
        Ok(())
    }

    fn decode(
        &self,
        value: &str,
        jwt_decoding_key: Arc<DecodingKey>,
        algorithm: Algorithm,
    ) -> Result<ClaimsType, Error> {
        let mut validation = Validation::new(algorithm);
        validation.validate_exp = true;
        validation.validate_nbf = false;
        validation.leeway = 0;
        validation.set_required_spec_claims(&["exp"]);

        decode::<ClaimsType>(value, &jwt_decoding_key, &validation)
            .map_err(|e| {
                log::error!("Failed to decode claims: {e:?}. {e}");
                Error::CantDecode
            })
            .map(|t| t.claims)
    }
}

impl<ClaimsType: Claims> CustomHeaderExtractor<ClaimsType> {
    async fn validate_login_path(&self, path: &str) -> bool {
        path.contains("/login")
    }
}
