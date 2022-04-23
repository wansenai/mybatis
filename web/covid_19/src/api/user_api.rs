use std::borrow::Borrow;

use crate::common::{response, dbconfig};
use crate::domain::user;

use actix_web :: {
    post,
    web,
    Result,
    Responder,
};
use mysql::prelude::Queryable;

#[post("/register")]
pub async fn register_user(data: web::Json<user::User>) ->  Result<impl Responder> {
    let mut conn = dbconfig::get_conn().unwrap();
    println!("连接情况: {:?}", conn);

    let result = conn.exec_drop("INSERT INTO user (id, username, password, name, sex, brithday, status) VALUES (?,?,?,?,?,?,?)", 
    (&data.id, &data.username, &data.password, &data.name, data.sex, &data.brithday, data.status,))
    .unwrap();

    let obj = response::SimpleResponse {
        code: 200,
        msg: String::from("创建用户成功"),
    };
    Ok(web::Json(obj))
}
