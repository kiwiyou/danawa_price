pub async fn send_message(bot_token: &str, chat_id: &str, message: &str) {
    let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}&parse_mode=MarkdownV2&disable_web_page_preview=true", bot_token, chat_id, message);
    let url: reqwest::Url = url.parse().unwrap();
    let res = reqwest::get(url).await.unwrap();
    println!("{}\n{}", res.status(), res.text().await.unwrap());
}

pub fn syntax(message: &str) -> String {
    message.replace("_", "\\_")
        .replace("*", "\\*")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace("~", "\\~")
        .replace("`", "\\`")
        .replace(">", "\\>")
        .replace("#", "\\#")
        .replace("+", "\\+")
        .replace("-", "\\-")
        .replace("=", "\\=")
        .replace("|", "\\|")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace(".", "\\.")
        .replace("!", "\\!")
}