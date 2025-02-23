use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct XmlRequestText {
    pub ToUserName: String,
    pub FromUserName: String,
    pub CreateTime: u64,
    pub MsgType: String,
    pub Content: Option<String>,
    pub MsgId: u64,
    pub MsgDataId: Option<String>,
    pub Idx: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct XmlRequestImage {
    pub ToUserName: String,
    pub FromUserName: String,
    pub CreateTime: u64,
    pub MsgType: String,
    pub PicUrl: Option<String>,
    pub MediaId: Option<String>,
    pub MsgId: u64,
    pub MsgDataId: Option<String>,
    pub Idx: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct XmlRequestVoice {
    pub ToUserName: String,
    pub FromUserName: String,
    pub CreateTime: u64,
    pub MsgType: String,
    pub MediaId: Option<String>,
    pub Format: Option<String>,
    pub MsgId: u64,
    pub MsgDataId: Option<String>,
    pub Idx: Option<String>,
    pub MediaId16K: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct XmlReplyText {
    pub ToUserName: String,
    pub FromUserName: String,
    pub CreateTime: u64,
    pub MsgType: String,
    pub Content: String,
}

impl XmlReplyText {
    pub fn new(from: &str, to: &str, msg: &str) -> Self {
        let build_cdata = |s: &str| Self::build_cdata_string(s); // 闭包来减少重复调用
        XmlReplyText {
            ToUserName: build_cdata(to),
            FromUserName: build_cdata(from),
            CreateTime: Self::get_timestamp(),
            MsgType: build_cdata("text"),
            Content: build_cdata(msg),
        }
    }

    /// Custom [Self::to_string] for wx
    pub fn to_string(&self) -> String {
        let xml = format!(
            r#"<xml>
            <ToUserName>{}</ToUserName>
            <FromUserName>{}</FromUserName>
            <CreateTime>{}</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content>{}</Content>
            </xml>"#,
            self.ToUserName, self.FromUserName, self.CreateTime, self.Content
        );
        xml.lines()
            .map(|line| line.trim_start())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn build_cdata_string(s: &str) -> String {
        format!("<![CDATA[{}]]>", s)
    }

    /// Return timestamp
    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur| dur.as_secs())
            .unwrap_or(0)
    }
}
