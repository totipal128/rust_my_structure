use actix_web::HttpResponse;
use serde::Serialize;


#[derive(Serialize)]
pub struct ResponseData<T> {
    message:String,
    data:T,
    code:i16,
}

impl <T>ResponseData<T> {
    pub fn new(message:String, code:i16, data:T) -> Self{
        ResponseData { message, data, code}
    }
}

pub enum NewResponseData<T> {
    Success(ResponseData<T>),
    BadRequest(ResponseData<T>),
    NotFound(ResponseData<T>),
    Unauthorized(ResponseData<T>),
}

pub fn inspect<T: Serialize + std::fmt::Debug>(response: NewResponseData<T>) -> HttpResponse {
    match response {
        NewResponseData::Success(res) => {
            HttpResponse::Ok().json(res)
        }
        NewResponseData::BadRequest(res) => {
            HttpResponse::BadRequest().json(res)
        }
        NewResponseData::NotFound(res) => {
            HttpResponse::NotFound().json(res)
        }
        NewResponseData::Unauthorized(res) => {
            HttpResponse::Unauthorized().json(res)
        }
    }
}
