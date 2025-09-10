#![allow(dead_code)]
use actix_web::HttpResponse;

use crate::helpers::{self, responses::responses::inspect};

pub async fn index () -> HttpResponse{
    let data = "isi data";
    let resp = helpers::responses::responses::ResponseData::new(
        "Welkowe first project rust".to_string(),
        200,
        data,
    );

    // get response enum
    inspect(helpers::responses::responses::NewResponseData::BadRequest(resp))
}