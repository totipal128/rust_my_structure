use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserInput {
    name: String,
}

#[derive(Serialize)]
struct UserResponse {
    message: String,
}

async fn create_user(user: web::Json<UserInput>) -> impl Responder {
    let resp = UserResponse {
        message: format!("Welcome, {}!", user.name),
    };
    HttpResponse::Ok().json(resp)
}

pub fn init(cfg:&mut web::ServiceConfig){
    cfg.route("/user", web::post().to(create_user));
}