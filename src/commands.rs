//! 命令处理模块

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use luo9_sdk::Bot;
use luo9_sdk::Msg;
use luo9_sdk::command::{Command, PrefixMode};

use crate::api::fetch_random_pig_images;

/// 冷却时间（秒）
const COOLDOWN_SECS: u64 = 5;

/// 用户冷却记录：user_id -> 上次触发时间
static COOLDOWN_MAP: Mutex<Option<HashMap<u64, Instant>>> = Mutex::new(None);

/// 解析相对路径为绝对路径
fn resolve(rel: &str) -> String {
    std::env::current_dir()
        .unwrap_or_default()
        .join(rel)
        .to_string_lossy()
        .replace('\\', "/")
}

/// 下载图片到临时文件，返回本地绝对路径
fn download_to_temp(url: &str) -> Option<String> {
    let dir = resolve("data/plugin_rollpig/temp");
    std::fs::create_dir_all(&dir).ok()?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_millis();

    // 从 URL 推断扩展名
    let ext = if url.contains(".png") { "png" } else { "jpg" };
    let filename = format!("{}.{}", timestamp, ext);
    let filepath = format!("{}/{}", dir, filename);

    let resp = reqwest::blocking::get(url).ok()?;
    let bytes = resp.bytes().ok()?;
    std::fs::write(&filepath, &bytes).ok()?;

    Some(filepath)
}

/// 检查用户是否在冷却期内，不在则更新时间戳并返回 true
fn check_cooldown(user_id: u64) -> bool {
    let now = Instant::now();
    let mut guard = COOLDOWN_MAP.lock().unwrap();
    let map = guard.get_or_insert_with(HashMap::new);

    if let Some(last_time) = map.get(&user_id) {
        if now.duration_since(*last_time).as_secs() < COOLDOWN_SECS {
            return false;
        }
    }

    map.insert(user_id, now);
    true
}

/// 处理私聊消息
pub fn handle_private_msg(user_id: u64, msg: &str) {
    handle_command(user_id, 0, msg, false);
}

/// 处理群消息
pub fn handle_group_msg(group_id: u64, user_id: u64, msg: &str) {
    handle_command(user_id, group_id, msg, true);
}

/// 处理命令
fn handle_command(user_id: u64, group_id: u64, msg: &str, is_group: bool) {
    let send = |cq: std::ffi::CString| {
        if is_group {
            let _ = Bot::send_group_msg(group_id, cq);
        } else {
            let _ = Bot::send_private_msg(user_id, cq);
        }
    };

    // 随机小猪（支持多个别名）
    let pig_aliases = ["随机小猪", "随机猪猪", "来点猪猪", "来点小猪"];
    let pig_cmd = pig_aliases.iter().find_map(|alias| Command::parse(msg, alias, PrefixMode::None));
    if let Some(cmd) = pig_cmd {
        // 检查冷却
        if !check_cooldown(user_id) {
            return;
        }

        let count = cmd
            .arg_at(0)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1)
            .min(20);

        let images = fetch_random_pig_images(count);

        if images.is_empty() {
            send(Msg::txt("猪圈空荡荡...").build());
            return;
        }

        for (url, _title) in &images {
            // 下载到临时文件再发送
            if let Some(filepath) = download_to_temp(url) {
                send(Msg::image(&filepath).build());
                // 延迟删除临时文件（等待发送完成）
                let path = filepath.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    let _ = std::fs::remove_file(&path);
                });
            }
        }
    }
}
