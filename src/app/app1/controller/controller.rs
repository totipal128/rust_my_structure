#![allow(dead_code)]
use actix_web::HttpResponse;

use crate::base::{self, responses::inspect};

pub async fn index () -> HttpResponse{
    let data = "isi data";
    let resp = base::responses::ResponseData::new(
        "Welkowe first project rust".to_string(),
        200,
        data,
    );

    // get response enum
    inspect(base::responses::NewResponseData::BadRequest(resp))
}