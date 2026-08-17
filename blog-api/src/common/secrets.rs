/*
 * 图床凭据静态加密（AES-256-GCM）：
 * - 密钥：32 字节随机数，首次使用时生成并写入 {config_dir}/picture_hosting.key（base64，0600）
 * - 密文格式：v1:<base64(nonce(12B) || ciphertext)>
 * - 丢失密钥文件 = 无法解密历史配置，需重新保存；请妥善备份该文件
 */
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use rand::RngCore;
use std::path::PathBuf;
use std::sync::LazyLock;

const KEY_FILE: &str = "picture_hosting.key";
const V1_PREFIX: &str = "v1:";

fn config_dir() -> PathBuf {
    std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./config"))
}

fn load_or_create_key() -> [u8; 32] {
    let path = config_dir().join(KEY_FILE);
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Ok(bytes) = B64.decode(text.trim()) {
            if bytes.len() == 32 {
                let mut key = [0u8; 32];
                key.copy_from_slice(&bytes);
                return key;
            }
        }
        tracing::warn!(
            "JWT 图床密钥文件 {} 内容无效，将重新生成（旧配置将无法解密）",
            path.display()
        );
    }

    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    let encoded = B64.encode(key);
    if let Err(e) = std::fs::create_dir_all(config_dir()) {
        tracing::error!("创建配置目录失败: {e}");
    }
    match std::fs::write(&path, encoded) {
        Ok(_) => tracing::info!("已生成图床密钥文件: {}", path.display()),
        Err(e) => tracing::error!("写入图床密钥文件失败: {e}"),
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }
    key
}

pub static PICTURE_HOSTING_KEY: LazyLock<[u8; 32]> = LazyLock::new(load_or_create_key);

/// 加密明文，返回 v1:<base64(nonce||ciphertext)>
pub fn encrypt_secret(plain: &str) -> Result<String, String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&PICTURE_HOSTING_KEY[..]));
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plain.as_bytes())
        .map_err(|e| format!("加密失败: {e}"))?;
    let mut blob = Vec::with_capacity(12 + ciphertext.len());
    blob.extend_from_slice(&nonce_bytes);
    blob.extend_from_slice(&ciphertext);
    Ok(format!("{V1_PREFIX}{}", B64.encode(blob)))
}

/// 解密 v1: 格式密文；返回 Err("非加密数据") 表示历史明文数据（调用方按明文处理）
pub fn decrypt_secret(stored: &str) -> Result<String, String> {
    let Some(rest) = stored.strip_prefix(V1_PREFIX) else {
        return Err("非加密数据".to_string());
    };
    let blob = B64
        .decode(rest.trim())
        .map_err(|e| format!("密文解码失败: {e}"))?;
    if blob.len() < 12 {
        return Err("密文格式错误".to_string());
    }
    let (nonce, ciphertext) = blob.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&PICTURE_HOSTING_KEY[..]));
    let plain = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|e| format!("解密失败（密钥文件与写入时不一致?）: {e}"))?;
    String::from_utf8(plain).map_err(|e| format!("解密结果非法: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let plain = r#"{"token":"ghp_xxxx","password":"p@ss","secretKey":"SK"}"#;
        let stored = encrypt_secret(plain).expect("encrypt 失败");
        assert!(stored.starts_with(V1_PREFIX), "密文应以 v1: 开头");
        // 密文中不得出现明文
        assert!(!stored.contains("ghp_xxxx"), "密文不应包含明文");
        assert!(!stored.contains("p@ss"), "密文不应包含明文");
        let decrypted = decrypt_secret(&stored).expect("decrypt 失败");
        assert_eq!(decrypted, plain);
        // 同一明文两次加密结果不同（随机 nonce）
        let stored2 = encrypt_secret(plain).expect("encrypt2 失败");
        assert_ne!(stored, stored2);
    }

    #[test]
    fn test_decrypt_legacy_plaintext_detected() {
        // 历史明文数据：不应解密成功，应返回"非加密数据"标记，调用方按明文兼容读取
        let err = decrypt_secret(r#"{"token":"legacy"}"#).unwrap_err();
        assert_eq!(err, "非加密数据");
    }

    #[test]
    fn test_decrypt_tampered_rejected() {
        let stored = encrypt_secret("secret-value").expect("encrypt 失败");
        let mut bytes = B64.decode(stored.trim_start_matches(V1_PREFIX)).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF; // 篡改最后一个字节
        let tampered = format!("{V1_PREFIX}{}", B64.encode(bytes));
        assert!(decrypt_secret(&tampered).is_err(), "篡改后的密文必须解密失败");
    }
}