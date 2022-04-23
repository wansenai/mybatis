use crate::common::response;
use crate::domain::user;
use crate::service::user_service;
use std::ops::Deref;

use actix_web :: {
    post,
    web,
    Result,
    Responder,
};


#[post("/register")]
pub async fn register_user(data: web::Json<user::User>) ->  Result<impl Responder> {

    let user = data.deref();
    let result_db = user_service::UserService::insert_user(user);

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
