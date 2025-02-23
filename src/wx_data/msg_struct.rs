use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct XmlRequestText {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: u64,
    pub msg_type: String,
    pub content: Option<String>,
    pub msg_id: u64,
    pub msg_data_id: Option<String>,
    pub idx: Option<String>,
}

#[derive(Deserialize, Debug)]
struct XmlRequestImage {
    to_user_name: String,
    from_user_name: String,
    create_time: u64,
    msg_type: String,
    pic_url: Option<String>,
    media_id: Option<String>,
    msg_id: u64,
    msg_data_id: Option<String>,
    idx: Option<String>,
}

#[derive(Deserialize, Debug)]
struct XmlRequestVoice {
    to_user_name: String,
    from_user_name: String,
    create_time: u64,
    msg_type: String,
    media_id: Option<String>,
    format: Option<String>,
    msg_id: u64,
    msg_data_id: Option<String>,
    idx: Option<String>,
    media_id16k: Option<String>,
}
