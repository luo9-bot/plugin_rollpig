//! 配置模块 - 从 data/plugin_rollpig/config.yaml 加载

use std::path::PathBuf;

/// 插件配置
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RollpigConfig {
    /// Vercel API 地址
    pub api_base: String,
    /// GitHub Raw 镜像前缀列表
    pub mirrors: Vec<String>,
}

impl Default for RollpigConfig {
    fn default() -> Self {
        RollpigConfig {
            api_base: "https://your-vercel-app.vercel.app".to_string(),
            mirrors: vec![
                "https://github.chenc.dev/".to_string(),
                "https://ghproxy.cfd/".to_string(),
                "https://ghproxy.cc/".to_string(),
                "https://gh-proxy.net/".to_string(),
            ],
        }
    }
}

/// 获取配置文件路径
fn config_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_default()
        .join("data")
        .join("plugin_rollpig")
        .join("config.yaml")
}

/// 加载配置，文件不存在时自动创建默认配置文件
pub fn load_config() -> RollpigConfig {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_yaml::from_str(&content).unwrap_or_default(),
        Err(_) => {
            let default_config = RollpigConfig::default();
            // 创建目录（如果不存在）
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            // 写入默认配置
            if let Ok(content) = serde_yaml::to_string(&default_config) {
                let _ = std::fs::write(&path, content);
            }
            default_config
        }
    }
}
