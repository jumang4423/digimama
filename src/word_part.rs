use rand::prelude::SliceRandom;

fn mama_like_emojis() -> Vec<String> {
    vec![
        "๐".to_string(),
        "๐๐".to_string(),
        "๐๐ถ".to_string(),
        "๐ฐ๐".to_string(),
        "๐ท๐".to_string(),
    ]
}

fn food_str() -> Vec<String> {
    vec![
        "็ๅผพใใซใใ".to_string(),
        "็ใใใซใฌใผ".to_string(),
        "ใใใใใๆใใซใฌใผ".to_string(),
        "ๅฝใใไปใใใณใใผใฐ".to_string(),
        "็ๅฃใญใ ใ้".to_string(),
        "ใใใฎใๅฅฝใฟ็ผใ".to_string(),
        "ใใผใใ".to_string(),
        "ๆๅฆปๅผๅฝ".to_string(),
    ]
}

pub fn surprises() -> Vec<String> {
    vec![
        "ใใ๐".to_string(),
        "ใพใใพใ".to_string(),
        "ใ๏ผ".to_string(),
        "ใใใชใฎใปใปใป๏ผ".to_string(),
        "ในใดใใใญใปใปใป๐ฐ๐".to_string(),
    ]
}

pub fn angry() -> Vec<fn() -> String> {
    vec![
        || -> String {
            format!(
                "{}ใ้จๅฑใฎใๆ้คใฏใฉใใใใฎ๏ผใกใใใจใใๅคงไบบใซใชใใชใใใใ",
                surprises().choose(&mut rand::thread_rng()).unwrap()
            )
        },
        || -> String { format!("ใใใใฏ้ฃในใชใใฎใญ๏ผ") },
    ]
}

pub fn support_word() -> Vec<fn() -> String> {
    vec![
        || -> String {
            format!(
                "ใใใฐใใใใฏใใใชใใฎใ๏ผ{}",
                mama_like_emojis().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
        || -> String {
            format!(
                "ไปๅคใฎๅค้ฃใ{}ใฏใใฉใใณในๆๅฎคไปฃๆ็ฎ0.5ใถๆๅใ{}",
                food_str().choose(&mut rand::thread_rng()).unwrap(),
                mama_like_emojis().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
        || -> String {
            format!(
                "{}ใ้ฃในใใฐใใใใซใชใใใ๏ผ",
                food_str().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
        || -> String {
            format!(
                "ใพใพใฏใใคใงใใใชใใฎ่ฆๆนใใปใปใป{}",
                mama_like_emojis().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
        || -> String {
            format!(
                "ใใคใงใๆฌ้ณใง่ฉฑใใใจใๅคงๅใชใฎใ๏ผ{}",
                mama_like_emojis().choose(&mut rand::thread_rng()).unwrap()
            )
            .to_string()
        },
    ]
}
