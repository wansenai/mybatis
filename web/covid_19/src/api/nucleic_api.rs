use crate::common::result::{ResultDefault, Result as IResult,};
use crate::domain::{
    nucleic_registe,
    nucleic_institution,
    nucleic_result,
};
use crate::service;
use crate::common::result_code;
use std::ops::Deref;

use actix_web :: {
    post,
    web,
    Result,
    Responder,
};

type NucleicRegiste = nucleic_registe::NucleicRegiste;
type NucleicInstitution = nucleic_institution::InstitutionObject;
type NucleicResult = nucleic_result::NucleicResultObject;
type ResultCode = result_code::ResultCode;

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

#[post("/institutionRegister")]
pub async fn institution_register(data: web::Json<NucleicInstitution>) -> Result<impl Responder> {
    let institution_data = data.deref();
    let result = service::NucleicInstitutionService::insert_nucleic_institution(institution_data);

    match result {
        true => {
            let success = ResultCode::success();
            let code = ResultCode::get_code(&success);
            let success_obj = <IResult<_> as ResultDefault>::new(code, String::from("机构注册成功"));

            Ok(web::Json(success_obj))
        },
        false => {
            let error_obj = <IResult<_> as ResultDefault>::fail();

            Ok(web::Json(error_obj))
        }
    }
}