use crate::env_handle::env::get_env;
use actix_web::web::Query;
use actix_web::{HttpResponse, Responder};
use log::debug;
use serde::Deserialize;
use sha1::{Digest, Sha1};

#[derive(Deserialize)]
pub struct QueryParams {
    signature: String,
    timestamp: String,
    nonce: String,
    echostr: String,
}

pub async fn wechat_verify(query: Query<QueryParams>) -> impl Responder {
    if check_signature(&query.signature, &query.timestamp, &query.nonce) {
        HttpResponse::Ok().body(query.echostr.clone()) // 验证通过返回 echo str
    } else {
        HttpResponse::Forbidden().finish() // authentic failed
    }
}

fn check_signature(signature: &str, timestamp: &str, nonce: &str) -> bool {
    let mut tmp_arr = vec![
        get_env("Token", "").expect("Token is not set"),
        timestamp.to_string(),
        nonce.to_string(),
    ];
    tmp_arr.sort(); // 字典序排序
    let tmp_str = tmp_arr.concat();

    let hasher = Sha1::digest(tmp_str.as_bytes());
    let tmp_str_hash = hex::encode(hasher);

    debug!("tmp str:   {}", tmp_str_hash);
    debug!("signature: {}", signature);
    tmp_str_hash == signature
}
