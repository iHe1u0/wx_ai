use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::{debug, error};

use crate::ai_handler::text_handler::send_message;
use crate::wx_data::msg_struct::{XmlReplyText, XmlRequestImage, XmlRequestText, XmlRequestVoice};
use chrono::Local;
use env_logger::Env;
use log::{info, LevelFilter};
use serde_xml_rs::from_str;
use std::io::Write;
use std::process::exit;

mod ai_handler;
mod env_handle;
mod wx_data;
mod wx_verify;

pub fn init_logger() {
    let env = Env::default().filter_or("RUST_LOG", "error");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            Ok({
                write!(
                    buf,
                    "{} {}\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.args()
                )
                .unwrap();
            })
        })
        .filter(None, LevelFilter::Debug)
        .init();
    info!("env_logger initialized.");
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn handle_msg(body: String) -> impl Responder {
    debug!("==== Handling Msg ====\n {}", body);
    let parsed_text: Result<XmlRequestText, _> = from_str(&body);
    let parsed_image: Result<XmlRequestImage, _> = from_str(&body);
    let parsed_voice: Result<XmlRequestVoice, _> = from_str(&body);

    if let Ok(parsed) = parsed_text {
        // WeChat has a limited time for reply message(5 seconds)
        // So start a new thread to post a message
        // And return empty body message in case error
        // for more information: https://developers.weixin.qq.com/doc/offiaccount/Message_Management/Passive_user_reply_message.html

        // let reply = handle_text(&parsed).await;
        // return HttpResponse::Ok().body(std::string::String::from(reply));
        let user = parsed.FromUserName;
        let msg = parsed.Content;
        let me = parsed.ToUserName;
        if msg.is_none() {
            let error_msg = XmlReplyText::new(me.as_str(), user.as_str(), "消息为空？");
            return HttpResponse::BadRequest().body(error_msg.to_string());
        };
        tokio::spawn(async move {
            let _ = send_message(user.as_str(), msg.unwrap().as_str()).await;
        });
        return HttpResponse::Ok().body("");
    } else if let Ok(_parsed) = parsed_image {
    } else if let Ok(_parsed) = parsed_voice {
    } else {
        error!("Failed to parse body: {:?}", body);
        // let error_msg = XmlReplyText::new(me.as_str(), user.as_str(), "暂不支持的消息类型");
        return HttpResponse::BadRequest().body("Failed to parse body");
    }

    HttpResponse::Ok().body("Hey there!")
}

async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route(
                "/msg",
                web::get().to(wx_verify::verify_token::wechat_verify),
            )
            .route("/msg", web::post().to(handle_msg))
    })
    .bind(("0.0.0.0", 10820))?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(_) = dotenvy::dotenv() {
        println!("Miss env var!");
        exit(1);
    }

    init_logger();

    // let list = ai_handler::openai::get_models().await;
    // debug!("{:?}", list);

    run_server().await.expect("Server failed.");

    Ok(())
}
