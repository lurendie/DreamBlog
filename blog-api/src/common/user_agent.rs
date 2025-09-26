use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// 浏览器匹配的正则表达式
static BROWSER_REGEXES: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    vec![
        (Regex::new(r"Firefox/([0-9.]+)").unwrap(), "Firefox"),
        (Regex::new(r"Chrome/([0-9.]+)").unwrap(), "Chrome"),
        (Regex::new(r"Safari/([0-9.]+)").unwrap(), "Safari"),
        (Regex::new(r"Edge/([0-9.]+)").unwrap(), "Edge"),
        (Regex::new(r"Opera/([0-9.]+)").unwrap(), "Opera"),
        (Regex::new(r"OPR/([0-9.]+)").unwrap(), "Opera"),
        (Regex::new(r"MSIE ([0-9.]+)").unwrap(), "Internet Explorer"),
        (
            Regex::new(r"Trident/.*rv:([0-9.]+)").unwrap(),
            "Internet Explorer",
        ),
    ]
});

// 操作系统匹配的正则表达式
static OS_REGEXES: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    vec![
        (Regex::new(r"Windows NT (10\.0)").unwrap(), "Windows 10"),
        (Regex::new(r"Windows NT (6\.3)").unwrap(), "Windows 8.1"),
        (Regex::new(r"Windows NT (6\.2)").unwrap(), "Windows 8"),
        (Regex::new(r"Windows NT (6\.1)").unwrap(), "Windows 7"),
        (Regex::new(r"Windows NT (6\.0)").unwrap(), "Windows Vista"),
        (Regex::new(r"Windows NT (5\.1)").unwrap(), "Windows XP"),
        (Regex::new(r"Windows NT ([0-9.]+)").unwrap(), "Windows"),
        (Regex::new(r"Mac OS X ([0-9_]+)").unwrap(), "macOS"),
        (Regex::new(r"Android ([0-9.]+)").unwrap(), "Android"),
        (Regex::new(r"Linux").unwrap(), "Linux"),
        (Regex::new(r"iPhone OS ([0-9_]+)").unwrap(), "iOS"),
        (Regex::new(r"iPad; CPU OS ([0-9_]+)").unwrap(), "iOS"),
    ]
});

/// 浏览器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserInfo {
    /// 浏览器名称
    pub name: String,
    /// 浏览器版本
    pub version: String,
}

/// 操作系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSInfo {
    /// 操作系统名称
    pub name: String,
    /// 操作系统版本
    pub version: String,
}

/// UserAgent解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgentInfo {
    /// 浏览器信息
    pub browser: BrowserInfo,
    /// 操作系统信息
    pub os: OSInfo,
    /// 原始UserAgent字符串
    pub user_agent: String,
}

pub struct UserAgent;

impl UserAgent {
    pub async fn parse_user_agent(user_agent: &str) -> UserAgentInfo {
        let browser = Self::parse_browser(user_agent);
        let os = Self::parse_os(user_agent);

        UserAgentInfo {
            browser,
            os,
            user_agent: user_agent.to_string(),
        }
    }

    /// 解析浏览器信息
    fn parse_browser(user_agent: &str) -> BrowserInfo {
        for (regex, name) in BROWSER_REGEXES.iter() {
            if let Some(caps) = regex.captures(user_agent) {
                let version = caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                return BrowserInfo {
                    name: name.to_string(),
                    version,
                };
            }
        }

        // 如果没有匹配到任何浏览器，尝试其他方法
        if user_agent.contains("Safari") && !user_agent.contains("Chrome") {
            return BrowserInfo {
                name: "Safari".to_string(),
                version: "Unknown".to_string(),
            };
        }

        BrowserInfo {
            name: "Unknown".to_string(),
            version: "Unknown".to_string(),
        }
    }

    /// 解析操作系统信息
    fn parse_os(user_agent: &str) -> OSInfo {
        for (regex, name) in OS_REGEXES.iter() {
            if let Some(caps) = regex.captures(user_agent) {
                let mut version = caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                // 处理macOS和iOS版本号中的下划线
                if (name == &"macOS" || name == &"iOS") && version.contains("_") {
                    version = version.replace("_", ".");
                }

                return OSInfo {
                    name: name.to_string(),
                    version,
                };
            }
        }

        // 尝试匹配其他操作系统
        if user_agent.contains("Mac") {
            return OSInfo {
                name: "macOS".to_string(),
                version: "Unknown".to_string(),
            };
        }

        if user_agent.contains("iPhone") || user_agent.contains("iPad") {
            return OSInfo {
                name: "iOS".to_string(),
                version: "Unknown".to_string(),
            };
        }

        OSInfo {
            name: "Unknown".to_string(),
            version: "Unknown".to_string(),
        }
    }

    // /// 返回默认的UserAgent信息
    // fn default_user_agent_info() -> UserAgentInfo {
    //     UserAgentInfo {
    //         browser: BrowserInfo {
    //             name: "Unknown".to_string(),
    //             version: "Unknown".to_string(),
    //         },
    //         os: OSInfo {
    //             name: "Unknown".to_string(),
    //             version: "Unknown".to_string(),
    //         },
    //         user_agent: "".to_string(),
    //     }
    // }
}
