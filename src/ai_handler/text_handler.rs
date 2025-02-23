use crate::wx_data::msg_handler::get_reply_msg;
use crate::wx_data::msg_struct::XmlRequestText;
use log::debug;

pub fn handle_text(data: &XmlRequestText) -> String {
    if data.Content.is_none() {
        return String::new();
    }
    debug!("{}", "-".repeat(12));
    debug!("User:{}", data.FromUserName);
    debug!("Time:{}", data.CreateTime);
    debug!("Text:{}", data.Content.clone().unwrap());
    get_reply_msg(
        &data.ToUserName,
        &data.FromUserName,
        &data.Content.clone().unwrap(),
    )
}
