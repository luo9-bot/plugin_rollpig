//! 命令处理模块

use luo9_sdk::Bot;
use luo9_sdk::Msg;
use luo9_sdk::command::{Command, PrefixMode};

use crate::api::fetch_random_pig_images;

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

        for (url, title) in &images {
            send(Msg::image(url).build());
        }
    }
}
