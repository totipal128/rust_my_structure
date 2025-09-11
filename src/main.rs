#![allow(dead_code)]

// get package rust
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use dotenvy::dotenv;
use scopeguard::defer;
use std::{env};
use crate::{base::databases::DB_CONNECTION, base::responses::inspect};
use helpers::route_logger::ROUTES;

// get module rust

mod schema;
mod helpers;
mod base;
mod app;

async fn index () -> HttpResponse{
    let data = "isi data";
    let resp = base::responses::ResponseData::new(
        "Welkowe first project rust".to_string(),
        200,
        data,
    );

    // get response enum
    inspect(base::responses::NewResponseData::Success(resp))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    defer!(drop(conn););
    // load env
    dotenv().ok();

    println!("====================================================================================================");
    println!("       ||=======               ==            ==      =================     ==================");
    println!("       ||      ===             ==            ==     ==================     ==================");
    println!("       ||       ====           ==            ==     ==                             ==");
    println!("       ||        ====          ==            ==     ==                             ==");
    println!("       ||       ===            ==            ==     =================              ==");
    println!("       ||========              ==            ==      =================             ==");
    println!("       ||========              ==            ==                    ==              ==");
    println!("       ||       ===            ==            ==                    ==              ==");
    println!("       ||        ====          ================     ==================             ==");
    println!("       ||          ====          ============       =================              ==");
    println!("====================================================================================================");
    println!("====================================================================================================");

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string()).parse::<u16>().expect("port format not found");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Cetak route sekali sebelum server jalan
    {
        let routes = ROUTES.lock().unwrap();
        println!("🚀 Registered routes:");
        println!("🚀 Registered routes:{:?}", &*routes);
        for route in routes.iter() {
            println!("📌{}", route);
        }
    }


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
