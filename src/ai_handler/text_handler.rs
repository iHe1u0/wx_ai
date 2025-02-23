use crate::wx_data::msg_struct::XmlRequestText;
use log::debug;

pub fn handle_text(data: &XmlRequestText) {
    if data.Content.is_none() {
        return;
    }
    debug!("{}", "-".repeat(12));
    debug!("User:{}", data.FromUserName);
    debug!("Time:{}", data.CreateTime);
    debug!("Text:{}", data.Content.clone().unwrap());
}
