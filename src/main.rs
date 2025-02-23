use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::debug;

use chrono::Local;
use env_logger::fmt::style::{AnsiColor, Color};
use env_logger::Env;
use log::{info, LevelFilter};
use std::io::Write;

mod wx_data;

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
    debug!("Access index");
    HttpResponse::Ok().body("Hello World!")
}

async fn verify() -> impl Responder {
    HttpResponse::Ok().body("true")
}

async fn handle_msg(body: String) -> impl Responder {
    debug!("handle_msg: {}", body);
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/msg", web::get().to(verify))
            .route("/msg", web::post().to(handle_msg))
    })
    .bind(("127.0.0.1", 10820))?
    .run()
    .await
}
