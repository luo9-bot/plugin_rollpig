//! 命令处理模块 - 对应原始__init__.py

use luo9_sdk::Bot;
use luo9_sdk::command::{Command, PrefixMode};
use std::ffi::CString;
use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

use crate::models::*;
use crate::api::*;
use crate::render::*;

lazy_static::lazy_static! {
    static ref USER_RECORDS: Mutex<HashMap<String, UserRecord>> = Mutex::new(HashMap::new());
}

/// 获取今日日期
fn get_today() -> String {
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let days = duration.as_secs() / 86400;
    format!("{}", days)
}

/// 检查用户今日记录
fn check_user_record(user_id: &str) -> Option<String> {
    let records = USER_RECORDS.lock().unwrap();
    let today = get_today();
    records.get(user_id)
        .filter(|r| r.date == today)
        .map(|r| r.pig_id.clone())
}

/// 保存用户记录
fn save_user_record(user_id: &str, pig_id: &str) {
    let mut records = USER_RECORDS.lock().unwrap();
    records.insert(user_id.to_string(), UserRecord {
        user_id: user_id.to_string(),
        pig_id: pig_id.to_string(),
        date: get_today(),
    });
}

/// 抽取今日小猪 (对应原始catch_today_pig)
fn catch_today_pig(user_id: &str) -> Pigsonality {
    if let Some(pig_id) = check_user_record(user_id) {
        if let Some(pig) = get_pigsonality_by_id(&pig_id) {
            return pig;
        }
    }
    
    let pigs = get_pigsonalities();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..pigs.len());
    let pig = pigs[index].clone();
    
    save_user_record(user_id, &pig.id);
    pig
}

/// 处理私聊消息
pub fn handle_private_msg(user_id: u64, msg: &str) {
    handle_command(user_id, 0, msg, false);
}

/// 处理群消息
pub fn handle_group_msg(group_id: u64, _user_id: u64, msg: &str) {
    handle_command(0, group_id, msg, true);
}

/// 处理命令
fn handle_command(user_id: u64, group_id: u64, msg: &str, is_group: bool) {
    let reply = |text: String| {
        if is_group {
            let _ = Bot::send_group_msg(group_id, CString::new(text).unwrap());
        } else {
            let _ = Bot::send_private_msg(user_id, CString::new(text).unwrap());
        }
    };
    
    let reply_image = |data: Vec<u8>| {
        let base64 = base64::encode(&data);
        let cq_code = format!("[CQ:image,file=base64://{}]", base64);
        if is_group {
            let _ = Bot::send_group_msg(group_id, CString::new(cq_code).unwrap());
        } else {
            let _ = Bot::send_private_msg(user_id, CString::new(cq_code).unwrap());
        }
    };
    
    // 今天是什么小猪
    if Command::parse(msg, "今天是什么小猪", PrefixMode::None).is_some() ||
       Command::parse(msg, "今日小猪", PrefixMode::None).is_some() ||
       Command::parse(msg, "本日小猪", PrefixMode::None).is_some() {
        let pig = catch_today_pig(&user_id.to_string());
        let img_data = generate_pig_card(&pig);
        reply_image(img_data);
        return;
    }
    
    // 随机小猪
    if let Some(cmd) = Command::parse(msg, "随机小猪", PrefixMode::None) {
        let count = if let Some(count_str) = cmd.arg_at(0) {
            count_str.parse::<usize>().unwrap_or(1).min(20)
        } else {
            1
        };
        
        let pigs = get_pigsonalities();
        let mut rng = rand::thread_rng();
        let mut result = Vec::new();
        for _ in 0..count.min(pigs.len()) {
            let index = rng.gen_range(0..pigs.len());
            result.push(pigs[index].clone());
        }
        
        if result.is_empty() {
            reply("猪圈空荡荡...".to_string());
            return;
        }
        
        if result.len() == 1 {
            let pig = &result[0];
            reply(format!("{}\n{}", pig.name, pig.image_url));
        } else {
            let mut msg = String::from("随机小猪：\n");
            for pig in &result {
                msg += &format!("\n{} ({})", pig.name, pig.id);
            }
            reply(msg);
        }
        return;
    }
    
    // 找猪
    if let Some(cmd) = Command::parse(msg, "找猪", PrefixMode::None).or_else(|| Command::parse(msg, "搜猪", PrefixMode::None)) {
        if !cmd.has_args() {
            reply("请输入关键词或图片ID~".to_string());
            return;
        }
        
        let keyword = cmd.args().join(" ");
        let found = search_pigsonalities(&keyword);
        
        if found.is_empty() {
            reply("你要找的猪仔离家出走了~".to_string());
            return;
        }
        
        if found.len() == 1 {
            let pig = &found[0];
            reply(format!("{}\n{}", pig.name, pig.image_url));
        } else {
            let mut msg = String::from("找到以下小猪：\n");
            for pig in found.iter().take(20) {
                msg += &format!("\n{} ({})", pig.name, pig.id);
            }
            reply(msg);
        }
        return;
    }
}
