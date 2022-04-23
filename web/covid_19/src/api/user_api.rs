use crate::common::response;
use crate::domain::user;
use crate::service;
use std::ops::Deref;

use actix_web :: {
    post,
    get,
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
            let success_obj = response::SimpleResponse {
                code: 200,
                msg: String::from("创建用户成功"),
            };
            Ok(web::Json(success_obj))
        },
        false =>  {
            let error_obj = response::SimpleResponse {
                code: 500,
                msg: String::from("创建用户失败，服务器异常"),
            };
            Ok(web::Json(error_obj))
        }
    }
}

#[get("/userLogin")]
pub async fn user_login(data: web::Json<user::User>) ->  Result<impl Responder> {

    let user = data.deref();

    let flag = service::UserService::user_login(user, &user.username, &user.password);

    match flag {
        true => {
            let success_obj = response::SimpleResponse {
                code: 200,
                msg: String::from("登陆成功"),
            };
            Ok(web::Json(success_obj))
        },
        false =>  {
            let error_obj = response::SimpleResponse {
                code: 500,
                msg: String::from("登陆失败，用户名或密码不正确"),
            };
            Ok(web::Json(error_obj))
        }
    }
}

