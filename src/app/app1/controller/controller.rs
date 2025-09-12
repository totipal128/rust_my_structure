#![allow(dead_code)]
use actix_web::HttpResponse;
use crate::app::app1::repository::users::get_all_users;
use crate::base::responses::ResponseData;
use crate::base::responses::NewResponseData;
use crate::inspect;

pub async fn index () -> HttpResponse{
    let data = "isi data";
    let resp = ResponseData::new(
        "Welkowe first project rust APP1".to_string(),
        200,
        data,
    );

    // get response enum
    inspect(NewResponseData::Success(resp))
}

pub async fn list_data () -> HttpResponse{
    let action= get_all_users();
    let resp= match action {
        Ok(data) => {
            ResponseData::new(
                "Get Data SUccess".to_string(),
                200,
                data
                )
            }
        Err(_) =>{
            let data_v = vec![];
            ResponseData::new(
                "Get Data Failed".to_string(),
                400,
                data_v,
                )
            }
    };
    

    inspect(NewResponseData::Success(resp))
}