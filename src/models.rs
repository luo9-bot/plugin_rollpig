//! 数据结构模块

/// 小猪性格 (对应原始Pigsonality)
#[derive(Clone, Debug)]
pub struct Pigsonality {
    pub id: String,
    pub name: String,
    pub description: String,
    pub analysis: String,
    pub image_url: String,
}

/// 用户记录 (对应原始UserRecord)
#[derive(Clone, Debug)]
pub struct UserRecord {
    pub user_id: String,
    pub pig_id: String,
    pub date: String,
}

/// PigHub API数据结构
#[derive(Clone, Debug, serde::Deserialize)]
pub struct PighubPig {
    pub id: String,
    pub title: String,
    pub thumbnail: String,
}

/// 获取小猪性格数据
pub fn get_pigsonalities() -> Vec<Pigsonality> {
    vec![
        Pigsonality {
            id: "1".to_string(),
            name: "运动小猪".to_string(),
            description: "一只热爱运动的小猪".to_string(),
            analysis: "你今天充满活力，适合进行体育锻炼！".to_string(),
            image_url: "https://pighub.top/data/sport_pig.jpg".to_string(),
        },
        Pigsonality {
            id: "2".to_string(),
            name: "吃货小猪".to_string(),
            description: "一只永远在吃的小猪".to_string(),
            analysis: "你今天食欲旺盛，适合享受美食！".to_string(),
            image_url: "https://pighub.top/data/foodie_pig.jpg".to_string(),
        },
        Pigsonality {
            id: "3".to_string(),
            name: "学霸小猪".to_string(),
            description: "一只戴着眼镜看书的小猪".to_string(),
            analysis: "你今天学习效率很高，适合钻研知识！".to_string(),
            image_url: "https://pighub.top/data/study_pig.jpg".to_string(),
        },
        Pigsonality {
            id: "4".to_string(),
            name: "摇滚小猪".to_string(),
            description: "一只弹着吉他的酷炫小猪".to_string(),
            analysis: "你今天充满创造力，适合表达自我！".to_string(),
            image_url: "https://pighub.top/data/rock_pig.jpg".to_string(),
        },
        Pigsonality {
            id: "5".to_string(),
            name: "宇航小猪".to_string(),
            description: "一只梦想飞向太空的小猪".to_string(),
            analysis: "你今天想象力丰富，适合探索未知！".to_string(),
            image_url: "https://pighub.top/data/space_pig.jpg".to_string(),
        },
    ]
}
