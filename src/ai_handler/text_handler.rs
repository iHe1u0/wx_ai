use crate::ai_handler::http_client::CLIENT;
use crate::env_handle::env::get_env;
use crate::wx_data::msg_handler::get_reply_msg;
use crate::wx_data::msg_struct::{JsonRequestData, XmlRequestText};
use crate::wx_verify::access_token::get_stable_token;
use log::{debug, error};

pub async fn handle_text(data: &XmlRequestText) -> String {
    if data.Content.is_none() {
        return String::new();
    }
    // debug!("User:{}", data.FromUserName);
    // debug!("Time:{}", data.CreateTime);
    // debug!("Text:{}", data.Content.clone().unwrap());
    get_reply_msg(
        &data.ToUserName,
        &data.FromUserName,
        &data.Content.clone().unwrap(),
    )
    .await
}

pub async fn send_message(user_id: &str, text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let msg_type = "text";
    let wx_token = get_stable_token().await.unwrap_or("".to_string());
    if wx_token.is_empty() {
        return Err(Box::from("token is empty"));
    }
    let post_url = format!(
        "https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}",
        wx_token
    );
    let client = CLIENT.lock().await.clone();
    // Build request data
    let request_data = JsonRequestData::new(user_id, msg_type, "已收到");
    // Post data
    let res = client.post(&post_url).json(&request_data).send().await?;
    //
    if res.status().is_success() {
        let response_data = res.text().await?;
        debug!("Post success:\n{}", response_data);
        Ok(response_data)
    } else {
        error!("Error sending message: {}", res.status());
        Err(format!("Request failed: {}", res.status()).into())
    }
}
