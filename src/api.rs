//! API模块 - 从 Vercel API 获取随机猪图，返回镜像 URL

use reqwest::blocking::Client;
use serde::Deserialize;
use std::sync::LazyLock;

use crate::config::{load_config, RollpigConfig};

static CONFIG: LazyLock<RollpigConfig> = LazyLock::new(load_config);

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap()
});

#[derive(Deserialize)]
struct ApiImage {
    title: String,
    url: String,
}

#[derive(Deserialize)]
struct ApiMultiResponse {
    images: Vec<ApiImage>,
}

/// 将 GitHub Raw URL 转为镜像 URL（取第一个镜像）
fn mirror_url(github_url: &str) -> String {
    if let Some(mirror) = CONFIG.mirrors.first() {
        format!("{}{}", mirror, github_url)
    } else {
        github_url.to_string()
    }
}

/// 获取随机猪图（返回镜像 URL + 标题）
pub fn fetch_random_pig_images(count: usize) -> Vec<(String, String)> {
    let count = count.clamp(1, 20);
    let api_url = format!("{}/api/random-pig?count={}", CONFIG.api_base, count);

    let images: Vec<ApiImage> = match HTTP_CLIENT.get(&api_url).send() {
        Ok(resp) if resp.status().is_success() => {
            if count == 1 {
                resp.json::<ApiImage>()
                    .map(|img| vec![img])
                    .unwrap_or_default()
            } else {
                resp.json::<ApiMultiResponse>()
                    .map(|r| r.images)
                    .unwrap_or_default()
            }
        }
        _ => return Vec::new(),
    };

    images
        .into_iter()
        .map(|img| (mirror_url(&img.url), img.title))
        .collect()
}
