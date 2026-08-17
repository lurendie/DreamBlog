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
    // 修复 fork 的路径不一致问题：
    // load_from_files 固定读 ./config，而 generate 写 args[1] 目录。
    // 启动前确保 ./config 存在并且密钥可从配置目录同步过来，避免重启时重新生成密钥导致全量掉线。
    ensure_jwt_key_files();
    let keys = JwtSigningKeys::load_or_create();
    let encoding_key = Arc::new(keys.encoding_key);
    let decoding_key = Arc::new(keys.decoding_key);

    let mut builder =
        SessionMiddlewareFactory::build(encoding_key.clone(), decoding_key, Algorithm::EdDSA);
    // Redis 可用时将会话存入 Redis，token 可被吊销（logout/改密后立即失效）；
    // 不可用时降级为无状态模式（token 仅依赖签名与过期时间）。
    let session_validation = match &*crate::app::REDIS_CLIENT {
        Some(pool) => {
            tracing::info!("JWT 会话存储启用 Redis，支持 token 吊销");
            builder = builder.with_redis_pool(pool.clone());
            true
        }
        None => {
            tracing::warn!("Redis 未启用，JWT 会话无法吊销，token 仅依赖签名与过期时间");
            builder = builder.with_storage(SessionStorage::new(
                Arc::new(NoopTokenStorage),
                encoding_key.clone(),
                Algorithm::EdDSA,
            ));
            false
        }
    };
    let (storage, factory) = builder
        .with_extractors(CustomExtractor::new(JWT_HEADER_NAME, session_validation))
        .finish();
    (storage, factory)
}

#[derive(Clone)]
struct NoopTokenStorage;

/// 启动前 JWT 密钥预检：
/// 1. 确保 `./config` 目录存在（fork 的 load_from_files 固定读取该路径）；
/// 2. 若 `./config` 下缺少密钥，但从启动参数指定的配置目录中能找到，则同步过来，
///    避免 fork 直接重新生成密钥导致所有在线 token 失效；
/// 3. Unix 下将密钥文件权限收紧为 0600。
fn ensure_jwt_key_files() {
    use std::path::Path;

    let config_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./config".to_string());
    let alt_dir = Path::new(&config_dir);
    let local_dir = Path::new("./config");

    if let Err(e) = std::fs::create_dir_all(local_dir) {
        tracing::error!("创建 JWT 密钥目录 ./config 失败: {e}");
    }

    for name in ["jwt-encoding.bin", "jwt-decoding.bin"] {
        let local_path = local_dir.join(name);
        if !local_path.exists() {
            let alt_path = alt_dir.join(name);
            if alt_path.exists() && alt_path != local_path {
                match std::fs::copy(&alt_path, &local_path) {
                    Ok(_) => tracing::info!(
                        "JWT 密钥 {} 已从 {} 同步到 ./config",
                        name,
                        alt_path.display()
                    ),
                    Err(e) => tracing::error!("同步 JWT 密钥 {} 失败: {}", name, e),
                }
            }
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for name in ["jwt-encoding.bin", "jwt-decoding.bin"] {
            let path = local_dir.join(name);
            if path.exists() {
                let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
            }
        }
    }
}

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
    pub fn new(name: &'static str, session_validation: bool) -> Extractors<AppClaims> {
        Extractors::new(
            vec![Arc::new(CustomHeaderExtractor::new(name, session_validation))],
            vec![],
        )
    }
}

#[derive(Debug)]
struct CustomHeaderExtractor<ClaimsType> {
    __ty: PhantomData<ClaimsType>,
    header_name: &'static str,
    /// 是否校验会话存储（Redis 模式）
    session_validation: bool,
}

impl<ClaimsType: Claims> CustomHeaderExtractor<ClaimsType> {
    pub fn new(header_name: &'static str, session_validation: bool) -> Self {
        Self {
            __ty: Default::default(),
            header_name,
            session_validation,
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
            // 无效/过期 token 按匿名处理，避免误伤前台公开接口
            Err(_) => return Ok(()),
        };
        // 会话校验（Redis 存储模式）：token 必须仍存在于存储中，吊销后立即失效
        if self.session_validation && self.validate(&decoded_claims, storage).await.is_err() {
            tracing::debug!("JWT 会话校验失败（token 已失效或不存在），按匿名处理");
            return Ok(());
        }
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
                tracing::debug!("Failed to decode claims: {e:?}. {e}");
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
