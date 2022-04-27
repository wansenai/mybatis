use crate::common::result::{ResultDefault, Result as IResult};
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

            let success_obj = <IResult<_> as ResultDefault>::success();

            Ok(web::Json(success_obj))
        },
        false =>  {
            let error_obj = <IResult<_> as ResultDefault>::fail();
            Ok(web::Json(error_obj))
        }
    }
}