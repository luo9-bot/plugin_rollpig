//! API模块 - 对应原始utils.py中的PigHub API

use reqwest;
use crate::models::{PighubPig, Pigsonality, get_pigsonalities};

/// 从PigHub随机获取小猪 (对应原始random_pigs)
pub async fn fetch_random_pigs(count: usize) -> Vec<PighubPig> {
    match reqwest::get("https://pighub.top/api/all-images").await {
        Ok(resp) => {
            match resp.json::<Vec<PighubPig>>().await {
                Ok(mut pigs) => {
                    let mut rng = rand::thread_rng();
                    let mut result = Vec::new();
                    for _ in 0..count.min(pigs.len()) {
                        let index = rng.gen_range(0..pigs.len());
                        result.push(pigs.remove(index));
                    }
                    result
                },
                Err(_) => Vec::new(),
            }
        },
        Err(_) => Vec::new(),
    }
}

/// 根据ID获取小猪
pub fn get_pigsonality_by_id(id: &str) -> Option<Pigsonality> {
    get_pigsonalities().into_iter().find(|p| p.id == id)
}

/// 根据关键词查找小猪
pub fn search_pigsonalities(keyword: &str) -> Vec<Pigsonality> {
    let keyword_lower = keyword.to_lowercase();
    get_pigsonalities().into_iter()
        .filter(|p| p.name.to_lowercase().contains(&keyword_lower) || p.id == keyword)
        .collect()
}
