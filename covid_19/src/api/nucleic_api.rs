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
    get,
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

#[post("/updateInstitutionById")]
pub async fn update_institution_by_id(data: web::Json<NucleicInstitution>) ->  Result<impl Responder> {
    let institution_data = data.deref();
    let result = service::NucleicInstitutionService::update_nucleic_institution(institution_data);

    match result {
        true => {
            let success = ResultCode::success();
            let code = ResultCode::get_code(&success);
            let success_obj = <IResult<_> as ResultDefault>::new(code, String::from("机构数据修改成功"));

            Ok(web::Json(success_obj))
        },
        false => {
            let error_obj = <IResult<_> as ResultDefault>::fail();

            Ok(web::Json(error_obj))
        }
    }
}


#[post("/insertNucleicResult")]
pub async fn insert_nucleic_result(data: web::Json<NucleicResult>) ->  Result<impl Responder> {
    let nucleic_result_data = data.deref();

    let id = &nucleic_result_data.id;
    let result_type = nucleic_result_data.result_type;
    let institution_id = &nucleic_result_data.institution_id;
    
    let mut registe_param_id = String::new();

    match &nucleic_result_data.registe_id {
        Some(registe_id) => registe_param_id.push_str(&registe_id),
        None => (),
    };

    let params = NucleicResult::other(id, result_type, institution_id, &registe_param_id);
    let result = service::NucleicResultService::insert_nucleic_result(&params);

    match result {
        true => {
            let success_obj = <IResult<_> as ResultDefault>::success();

            Ok(web::Json(success_obj))
        },
        false => {
            let error_obj = <IResult<_> as ResultDefault>::fail();

            Ok(web::Json(error_obj))
        }
    }
}

#[get("/getNucleicInstitutionByRegion/{region}")]
pub async fn get_nucleic_institution_by_region(path: web::Path<String>) -> Result<impl Responder> {
    let region = path.into_inner();

    let result = <NucleicInstitution as service::NucleicInstitutionService>::query_nucleic_institution_byregion(&region);
    
    if result.len() <= 0 {

        let nodata_obj = IResult::no_data();
        return Ok(web::Json(nodata_obj))
    }

    let success_obj = IResult::success_data(result);

    Ok(web::Json(success_obj))
}

#[get("/getNucleicInstitutionByName/{name}")]
pub async fn get_nucleic_institution_by_name(path: web::Path<String>) -> Result<impl Responder> {
    let name = path.into_inner();

    let result = <NucleicInstitution as service::NucleicInstitutionService>::query_nucleic_institution_byname(&name);
    
    if result.len() <= 0 {

        let nodata_obj = IResult::no_data();
        return Ok(web::Json(nodata_obj))
    }

    let success_obj = IResult::success_data(result);

    Ok(web::Json(success_obj))
}