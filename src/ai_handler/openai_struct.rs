use serde::{Deserialize, Serialize};

/// For get models
#[derive(Serialize, Deserialize, Debug)]
pub struct ModelList {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub owned_by: String,
}

/// For openai request
#[derive(Serialize, Deserialize, Clone)]
pub struct RequestMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct RequestBody {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub temperature: f32,
}

/// For openai response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
    stats: Option<std::collections::HashMap<String, serde_json::Value>>, // None if stats is empty
    system_fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    index: i32,
    logprobs: Option<serde_json::Value>, // None if logprobs is empty
    finish_reason: String,
    pub message: ResponseMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMessage {
    pub(crate) role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}
