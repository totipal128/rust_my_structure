#![allow(dead_code)]

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use dotenvy::dotenv;
use serde::{Serialize};
use std::{env};

mod base;

#[derive(Serialize)]
pub struct ResponseData<T> {
    message:String,
    data:T,
    code:i16,
}

async fn index () -> HttpResponse{
    let data = "isi data";
    let resp = ResponseData{
        message:"Welkowe first project rust".to_string(),
        code:200,
        data:data,
    };

    HttpResponse::Ok().json(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env
    dotenv().ok();

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string()).parse::<u16>().expect("port format not found");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        // .wrap(Logger::new("%a %r %s %b %{User-Agent}i"))
        .route("/", web::get().to(index))
        .configure(base::route::route::init)
    })
    .bind((host, port))?
    .run()
    .await
}
