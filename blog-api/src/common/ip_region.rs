use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::LazyLock;

use actix_web::HttpRequest;

use crate::common::ip_value::ToUIntIP;

pub struct IpRegion;

fn default_detect_xdb_file() -> Result<String, Box<dyn Error>> {
    let prefix = "./".to_owned();
    for recurse in 1..4 {
        let filepath = prefix.repeat(recurse) + "data/ip2region.xdb";
        if Path::new(filepath.as_str()).exists() {
            return Ok(filepath);
        }
    }
    Err("default filepath not find the xdb file, so you must set xdb_filepath".into())
}
/// 加载失败的场合不 panic，而是缓存 None；调用方通过 `unwrap_or_default()` 得到空字符串
static CACHE: LazyLock<Option<Vec<u8>>> = LazyLock::new(|| {
    let filepath = match default_detect_xdb_file() {
        Ok(filepath) => filepath,
        Err(e) => {
            tracing::warn!("ip2region 数据库未加载: {e}");
            return None;
        }
    };
    let mut file = match File::open(&filepath) {
        Ok(file) => file,
        Err(e) => {
            tracing::warn!("ip2region 数据库未加载: {e}");
            return None;
        }
    };
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        tracing::warn!("ip2region 数据库未加载: {e}");
        return None;
    }
    Some(buffer)
});
impl IpRegion {
    const HEADER_INFO_LENGTH: usize = 256;
    const VECTOR_INDEX_COLS: usize = 256;
    const VECTOR_INDEX_SIZE: usize = 8;
    const SEGMENT_INDEX_SIZE: usize = 14;
    const VECTOR_INDEX_LENGTH: usize = 512 * 1024;

    const _XDB_FILEPATH_ENV: &str = "XDB_FILEPATH";

    /// check https://mp.weixin.qq.com/s/ndjzu0BgaeBmDOCw5aqHUg for details
    pub fn search_by_ip<T>(ip: T) -> Result<String, Box<dyn Error>>
    where
        T: ToUIntIP + Display,
    {
        let Some(cache) = CACHE.as_ref() else {
            return Err("ip2region 数据库未加载".into());
        };
        let ip = ip.to_u32_ip()?;
        let il0 = ((ip >> 24) & 0xFF) as usize;
        let il1 = ((ip >> 16) & 0xFF) as usize;
        let idx = Self::VECTOR_INDEX_SIZE * (il0 * Self::VECTOR_INDEX_COLS + il1);
        let start_point = idx;
        let vector_cache = Self::get_vector_index_cache();
        let start_ptr = Self::get_block_by_size(vector_cache, start_point, 4);
        let end_ptr = Self::get_block_by_size(vector_cache, start_point + 4, 4);
        let mut left: usize = 0;
        let mut right: usize = (end_ptr - start_ptr) / Self::SEGMENT_INDEX_SIZE;

        while left <= right {
            let mid = (left + right) >> 1;
            let offset = start_ptr + mid * Self::SEGMENT_INDEX_SIZE;
            let buffer_ip_value = &cache[offset..offset + Self::SEGMENT_INDEX_SIZE];
            let start_ip = Self::get_block_by_size(buffer_ip_value, 0, 4);
            if ip < (start_ip as u32) {
                right = mid - 1;
            } else if ip > (Self::get_block_by_size(buffer_ip_value, 4, 4) as u32) {
                left = mid + 1;
            } else {
                let data_length = Self::get_block_by_size(buffer_ip_value, 8, 2);
                let data_offset = Self::get_block_by_size(buffer_ip_value, 10, 4);
                let result =
                    String::from_utf8(cache[data_offset..(data_offset + data_length)].to_vec());
                return Ok(result?);
            }
        }
        Err("not matched".into())
    }

    /// it will check ../data/ip2region.xdb, ../../data/ip2region.xdb, ../../../data/ip2region.xdb

    pub fn get_vector_index_cache() -> &'static [u8] {
        static EMPTY: &[u8] = &[0u8; 0];
        match CACHE.as_ref() {
            Some(cache) => &cache
                [Self::HEADER_INFO_LENGTH..(Self::HEADER_INFO_LENGTH + Self::VECTOR_INDEX_LENGTH)],
            None => EMPTY,
        }
    }

    pub fn get_block_by_size(bytes: &[u8], offset: usize, length: usize) -> usize {
        let mut result: usize = 0;
        for (index, value) in bytes[offset..offset + length].iter().enumerate() {
            result += usize::from(*value) << (index << 3);
        }
        result
    }

    /// 获取真实的客户端IP地址，考虑代理和转发的情况
    /// `trust_proxy` 为 false 时（默认）仅信任 TCP 对端地址，防止客户端伪造转发头；
    /// 仅当服务部署在可信反向代理（如 Nginx）之后时置为 true。
    pub fn get_real_client_ip(req: &HttpRequest, trust_proxy: bool) -> String {
        if !trust_proxy {
            return req
                .connection_info()
                .peer_addr()
                .unwrap_or("unknown")
                .to_string();
        }

        // 信任代理模式：按优先级尝试获取IP地址
        let headers = req.headers();

        // 1. 首先检查 X-Forwarded-For 头
        if let Some(x_forwarded_for) = headers.get("X-Forwarded-For") {
            if let Ok(x_forwarded_for_str) = x_forwarded_for.to_str() {
                // X-Forwarded-For 可能包含多个IP，第一个是真实的客户端IP
                let ips: Vec<&str> = x_forwarded_for_str.split(',').collect();
                if !ips.is_empty() {
                    let ip = ips[0].trim();
                    if valid_ip_like(ip) {
                        return ip.to_string();
                    }
                }
            }
        }

        // 2. 检查 X-Real-IP 头
        if let Some(x_real_ip) = headers.get("X-Real-IP") {
            if let Ok(x_real_ip_str) = x_real_ip.to_str() {
                let ip = x_real_ip_str.trim();
                if valid_ip_like(ip) {
                    return ip.to_string();
                }
            }
        }

        // 3. 检查 Proxy-Client-IP 头
        if let Some(proxy_client_ip) = headers.get("Proxy-Client-IP") {
            if let Ok(proxy_client_ip_str) = proxy_client_ip.to_str() {
                let ip = proxy_client_ip_str.trim();
                if valid_ip_like(ip) {
                    return ip.to_string();
                }
            }
        }

        // 4. 检查 WL-Proxy-Client-IP 头
        if let Some(wl_proxy_client_ip) = headers.get("WL-Proxy-Client-IP") {
            if let Ok(wl_proxy_client_ip_str) = wl_proxy_client_ip.to_str() {
                let ip = wl_proxy_client_ip_str.trim();
                if valid_ip_like(ip) {
                    return ip.to_string();
                }
            }
        }

        // 5. 最后从连接信息中获取IP
        let conn_info = req.connection_info();
        // 如果都无法获取，返回unknown
        //"unknown".to_string()
        return conn_info.peer_addr().unwrap_or("unknown").to_string();
    }
}

/// 校验候选取值是否为合法的 IP 字面量（支持 [v6]:port 与 v4:port 形式），
/// 防止伪造转发头携带任意非 IP 值污染日志/统计。
fn valid_ip_like(s: &str) -> bool {
    if s.trim().is_empty() {
        return false;
    }
    if let Ok(_) = s.parse::<std::net::IpAddr>() {
        return true;
    }
    // 形式 [v6]:port 或 v4:port：剥离最后一个冒号后的端口再解析
    if let Some(colon_pos) = s.rfind(':') {
        let candidate = s[..colon_pos].trim_start_matches('[').trim_end_matches(']');
        if candidate.parse::<std::net::IpAddr>().is_ok() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::str::FromStr;

    use super::*;

    ///test all types find correct
    #[test]
    fn test_multi_type_ip() {
        //searcher_init(Some("./data/ip2region.xdb".to_string()));

        let ip_str = IpRegion::search_by_ip("2.0.0.0").unwrap();
        println!("{ip_str}");
        IpRegion::search_by_ip("32").unwrap();
        IpRegion::search_by_ip(4294408949).unwrap();
        IpRegion::search_by_ip(Ipv4Addr::from_str("1.1.1.1").unwrap()).unwrap();
    }
}
