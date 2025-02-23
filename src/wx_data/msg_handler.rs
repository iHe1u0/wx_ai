use crate::wx_data::msg_struct::XmlReplyText;
use std::option::Option;

/// Get a reply message when the user sends a message
pub fn get_reply_msg(from: &str, to: &str, msg: &str) -> String {
    // Use get_reply_msg_ai to get AI reply message, return error message if AI dead...
    let reply = get_reply_msg_ai(msg).unwrap_or_else(|| "AI built message failed".to_string());
    XmlReplyText::new(from, to, &reply).to_string()
}

fn get_reply_msg_ai(msg: &str) -> Option<String> {
    Some(String::from(msg))
}
