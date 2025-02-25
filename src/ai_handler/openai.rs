use crate::ai_handler::http_client::CLIENT;
use crate::ai_handler::openai_struct::{ModelList, OpenAiResponse, RequestBody, RequestMessage};
use crate::env_handle::env::get_env;
use log::debug;
use std::error::Error;

/// Asynchronously fetches the list of models from the OpenAI API.
///
/// This function constructs the URL for the OpenAI models endpoint by retrieving the
/// `OPENAI_HOST` environment variable (or using the default `"https://api.openai.com"` if not set).
/// It then sends a GET request to this URL, parses the response JSON into a `ModelList` object,
/// and returns it as the result. If any error occurs during the HTTP request or JSON parsing,
/// the function returns the error as a `Box<dyn std::error::Error>`.
///
/// # Returns
/// - `Ok(ModelList)` if the request and JSON parsing are successful.
/// - `Err(Box<dyn std::error::Error>)` if there is any failure during the HTTP request or parsing.
pub async fn get_models() -> Result<ModelList, Box<dyn Error>> {
    let option_api_url = get_env("OpenAiHost", "https://api.openai.com");
    // Check api_url
    let api_url = match option_api_url {
        Some(url) => url,
        None => return Err("OpenAiHost is not set".into()),
    };

    let models_url = format!("{}/v1/models", api_url);
    let models: ModelList = reqwest::get(models_url).await?.json().await?;
    Ok(models)
}

#[allow(unused)]
pub async fn get_reply(
    model: &str,
    messages: Vec<RequestMessage>,
    temperature: Option<f32>,
) -> Result<String, Box<dyn Error>> {
    let option_api_url = get_env("OpenAiHost", "https://api.openai.com");
    let api_url = match option_api_url {
        Some(url) => url,
        None => return Err("OpenAiHost is not set".into()),
    };
    let chat_url = format!("{}/v1/chat/completions", api_url);
    let client = CLIENT.lock().await.clone();
    let option_model_name = get_env("ModelName", "");
    let model_name = match option_model_name {
        Some(url) => url,
        None => return Err("ModelName is not set".into()),
    };
    let request_body = RequestBody {
        model: model_name,
        messages,
        temperature: temperature.unwrap_or(0.7),
    };
    let response = client.post(&chat_url).json(&request_body).send().await?;
    let rep: OpenAiResponse = response.json().await?;

    rep.choices
        .first()
        .map(|choice| {
            let rep_text = choice.message.content.clone();
            rep_text
        })
        .ok_or_else(|| {
            debug!("No response from API");
            "No response from API".into()
        })
}
