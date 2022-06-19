use rand::seq::SliceRandom; // 0.8.0

enum MamaMood {
    angry,
    sad,
    happy,
    bored,
    excited,
    sleepy,
    support,
    sick,
    nerdy,
    sexy,
}

pub fn gen_mama_str() -> String {
    let mut rng = rand::thread_rng();
    let moods = [
        MamaMood::angry,
        MamaMood::sad,
        MamaMood::happy,
        MamaMood::bored,
        MamaMood::excited,
        MamaMood::sleepy,
        MamaMood::support,
        MamaMood::sick,
        MamaMood::nerdy,
        MamaMood::sexy
    ];
    let cur_mood = moods.choose(&mut rng).unwrap();

    // TODO: moods to word_part func
    match cur_mood {
        MamaMood::angry => gen_angry_str(),
        MamaMood::sad => "I'm sad!".to_string(),
        MamaMood::happy => "I'm happy!".to_string(),
        MamaMood::bored => "I'm bored!".to_string(),
        MamaMood::excited => "I'm excited!".to_string(),
        MamaMood::sleepy => "I'm sleepy!".to_string(),
        MamaMood::support => "I'm supporting!".to_string(),
        MamaMood::sick => "I'm sick!".to_string(),
        MamaMood::nerdy => "I'm nerdy!".to_string(),
        MamaMood::sexy => "I'm sexy!".to_string(),
    }
}

pub fn gen_angry_str() -> String {
    "".to_string()
}