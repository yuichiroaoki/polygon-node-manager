use crate::request;
use crate::utils::get_env;

pub async fn send_notification(text: &str) {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        get_env("API_KEY"),
        get_env("CHAT_ID"),
        text
    );

    let res = request::send_get_request(&url).await;
    println!("res: {}", res.unwrap());
}
