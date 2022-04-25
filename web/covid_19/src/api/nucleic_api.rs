use crate::common::response;
use crate::domain::nucleic_registe;
use crate::service;

use std::ops::Deref;

use actix_web :: {
    post,
    web,
    Result,
    Responder,
};

type NucleicRegiste = nucleic_registe::NucleicRegiste;

#[post("/nucleicRegister")]
pub async fn nucleic_register(data: web::Json<NucleicRegiste>) ->  Result<impl Responder> {

    let nucleic = data.deref();
    let result_db = service::NucleicService::nucleic_registe(nucleic);

    match result_db {
        true => {
            let success_obj = response::SimpleResponse {
                code: 200,
                msg: String::from("个人核酸登记成功"),
            };
            Ok(web::Json(success_obj))
        },
        false =>  {
            let error_obj = response::SimpleResponse {
                code: 500,
                msg: String::from("个人核酸登记失败，服务器异常"),
            };
            Ok(web::Json(error_obj))
        }
    }
}