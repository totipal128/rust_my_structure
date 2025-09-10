#![allow(dead_code)]
use actix_web::{web};

pub mod app1;

pub fn init(cfg:&mut web::ServiceConfig){
    cfg.configure(app1::init);
}