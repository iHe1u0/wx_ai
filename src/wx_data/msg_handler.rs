use crate::ai_handler::openai::{get_models, get_reply};
use crate::ai_handler::openai_struct::RequestMessage;
use crate::wx_data::msg_struct::XmlReplyText;
use log::debug;
use std::option::Option;

/// Get a reply message when the user sends a message
pub async fn get_reply_msg(from: &str, to: &str, msg: &str) -> String {
    // Use get_reply_msg_ai to get AI reply message, return error message if AI dead...
    let reply = get_reply_msg_ai(msg)
        .await
        .unwrap_or_else(|| "AI built message failed".to_string());
    debug!("===========reply start===========");
    debug!("{:?}", &reply);
    XmlReplyText::new(from, to, &reply).to_string()
}

async fn get_reply_msg_ai(msg: &str) -> Option<String> {
    let models = get_models().await;
    if let Err(_) = models {
        return None;
    }
    let model_list = models.unwrap();
    if model_list.data.is_empty() {
        debug!("No available models found");
        return None;
    }
    if let None = model_list.data.first() {
        return None;
    }
    let model_id = &model_list.data.first()?.id;
    // Now let's build request messages
    let mut msgs = Vec::new();

    msgs.push(RequestMessage {
        role: "user".to_string(),
        content: msg.to_string(),
    });

    let reply_msg = get_reply(model_id, msgs, None).await;

    let result = match reply_msg {
        Ok(reply) => Some(reply),
        Err(err) => Some(err.to_string()),
    };
    result
}
