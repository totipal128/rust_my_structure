#![allow(dead_code)]
use actix_web::{ web::{self}};
use crate::{helpers::route_logger::RouterPrintExt};

pub mod controller;
pub mod models;

pub fn init(cfg:&mut web::ServiceConfig){
    let base = "/app1";
    cfg.service(
        web::scope("/app1")
        .route_print("", "get", controller::controller::index, base)
        .route_print("/get", "get", controller::controller::index, base)
        .route_print("post", "get", controller::controller::index, base)
        
    );
}