const system_about: &str = "ママっぽい言葉を生成するコマンドラインツール";

pub fn gen_help_text() -> Vec<String> {
    vec![
        String::from("about:"),
        String::from(format!("  digimama - {}", system_about)),
        String::from(""),
        String::from("options:"),
        String::from("  -h, --help    print this help text"),
    ]
}