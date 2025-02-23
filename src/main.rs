use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::{debug, error};

use crate::ai_handler::text_handler::handle_text;
use crate::wx_data::msg_struct::{XmlRequestImage, XmlRequestText, XmlRequestVoice};
use chrono::Local;
use dotenv_codegen::dotenv;
use env_logger::fmt::style::{AnsiColor, Color};
use env_logger::Env;
use log::{info, LevelFilter};
use serde_xml_rs::from_str;
use std::io::Write;
use std::process::exit;

mod ai_handler;
mod wx_data;
mod wx_verify;

pub fn init_logger() {
    let env = Env::default().filter_or("RUST_LOG", "debug");
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
    let now = Local::now();
    debug!("Access at {}", now.format("%H:%M:%S"));
    HttpResponse::Ok().body("Hello World!")
}

async fn handle_msg(body: String) -> impl Responder {
    let parsed_text: Result<XmlRequestText, _> = from_str(&body);
    let parsed_image: Result<XmlRequestImage, _> = from_str(&body);
    let parsed_voice: Result<XmlRequestVoice, _> = from_str(&body);

    if let Ok(parsed) = parsed_text {
        let reply = handle_text(&parsed);
        debug!("{:?}", reply);
        return HttpResponse::Ok().body(std::string::String::from(reply));
    } else if let Ok(parsed) = parsed_image {
    } else if let Ok(parsed) = parsed_voice {
    } else {
        error!("Failed to parse body: {:?}", body);
        return HttpResponse::BadRequest().body("暂不支持的消息类型");
    }

    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(_) = dotenvy::dotenv() {
        println!("Miss env var!");
        exit(1);
    }

    init_logger();

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
