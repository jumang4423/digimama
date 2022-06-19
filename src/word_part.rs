use rand::prelude::SliceRandom;

fn mama_like_emojis() -> Vec<String> {
    vec![
        "💔".to_string(),
        "💔💔".to_string(),
        "🍙🐶".to_string(),
        "🍰🍙".to_string(),
        "🌷💓".to_string(),
    ]
}

fn food_str() -> Vec<String> {
    vec![
        "爆弾おにぎり".to_string(),
        "牛すじカレー".to_string(),
        "じゃがいも抜きカレー".to_string(),
        "当たり付きハンバーグ".to_string(),
        "甘口キムチ鍋".to_string(),
        "いかのお好み焼き".to_string(),
        "そーめん".to_string(),
        "愛妻弁当".to_string(),
    ]
}

pub fn surprises() -> Vec<String> {
    vec![
        "あら💔".to_string(),
        "まあまあ".to_string(),
        "え！".to_string(),
        "そうなの・・・？".to_string(),
        "スゴいわね・・・🍰🍙".to_string(),
    ]
}

pub fn angry() -> Vec<fn() -> String> {
    vec![
        || -> String {
            format!(
                "{}、部屋のお掃除はどうしたの？ちゃんとした大人になれないわよ。",
                surprises().choose(&mut rand::thread_rng()).unwrap()
            )
        },
        || -> String { format!("トマトは食べないのね！") },
    ]
}

pub fn support_word() -> Vec<fn() -> String> {
    vec![
        || -> String {
            format!(
                "がんばりすぎはよくないのよ！{}",
                mama_like_emojis().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
        || -> String {
            format!(
                "今夜の夕食、{}はフラダンス教室代換算0.5ヶ月分よ{}",
                food_str().choose(&mut rand::thread_rng()).unwrap(),
                mama_like_emojis().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
        || -> String {
            format!(
                "{}を食べればげんきになるわよ！",
                food_str().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
        || -> String {
            format!(
                "ままはいつでもあなたの見方よ・・・{}",
                mama_like_emojis().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
        || -> String {
            format!(
                "いつでも本音で話すことが大切なのよ！{}",
                mama_like_emojis().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
    ]
}
