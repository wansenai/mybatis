use crate::common::result::{ResultDefault, Result as myResult};
use crate::domain::user;
use crate::service;

use std::ops::Deref;

use actix_web :: {
    post,
    web,
    Result,
    Responder,
};


#[post("/userRegister")]
pub async fn user_register(data: web::Json<user::User>) ->  Result<impl Responder> {

    let user = data.deref();
    let result_db = service::UserService::insert_user(user);

    match result_db {
        true => {

            let success_obj = <ResultDefault as myResult>::success();

            Ok(web::Json(success_obj))
        },
        false =>  {
            let error_obj = <ResultDefault as myResult>::fail();

            Ok(web::Json(error_obj))
        }
    }
}

#[post("/userLogin")]
pub async fn user_login(data: web::Json<user::User>) ->  Result<impl Responder> {

    let user = data.deref();

    let flag = service::UserService::user_login(user, &user.username, &user.password);

    match flag {
        true => {

            let success_obj = <ResultDefault as myResult>::success();

            Ok(web::Json(success_obj))
        },
        false =>  {
            let error_obj = <ResultDefault as myResult>::fail();

            Ok(web::Json(error_obj))
        }
    }
}

