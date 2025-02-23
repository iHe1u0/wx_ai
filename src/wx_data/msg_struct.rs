use serde::Deserialize;

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

// <xml>
// <ToUserName><![CDATA[toUser]]></ToUserName>
// <FromUserName><![CDATA[fromUser]]></FromUserName>
// <CreateTime>12345678</CreateTime>
// <MsgType><![CDATA[text]]></MsgType>
// <Content><![CDATA[你好]]></Content>
// </xml>
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct XmlReplyText {
    pub ToUserName: String,
    pub FromUserName: String,
    pub CreateTime: u64,
    pub MsgType: String,
    pub Content: String,
}
