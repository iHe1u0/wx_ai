use crate::ai_handler::http_client::CLIENT;
use crate::env_handle::env::get_env;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct StableTokenRequest {
    grant_type: String,
    appid: String,
    secret: String,
    force_refresh: bool,
}

impl StableTokenRequest {
    pub fn new(app_id: String, secret: String) -> StableTokenRequest {
        StableTokenRequest {
            grant_type: "client_credential".to_string(),
            appid: app_id,
            secret,
            force_refresh: false,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct StableTokenResponse {
    access_token: String,
    expires_in: u64,
}

pub async fn get_stable_token() -> Result<String, Box<dyn Error>> {
    let api_url = "https://api.weixin.qq.com/cgi-bin/stable_token";
    let client = CLIENT.lock().await.clone();
    let app_id = get_env("AppId", "").unwrap_or("".to_string());
    let secret = get_env("Secret", "").unwrap_or("".to_string());
    let request_data = StableTokenRequest::new(app_id, secret);

    let res = client
        .post(api_url)
        .json(&request_data)
        .body(request_data.to_json())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        let response: StableTokenResponse = res.json().await?;
        Ok(response.access_token)
    } else {
        Err(format!("Failed to get access token: {}", res.text().await.unwrap()).into())
    }
}
