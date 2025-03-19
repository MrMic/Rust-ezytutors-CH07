use std::env;

use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8082".to_string());
    println!("listening on {} - Open browser and visit", addr);
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(addr)?
    .run()
    .await
}
