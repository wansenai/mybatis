use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    // id 主键
    #[serde(default)]
    pub id: String,
    // 用户名
    #[serde(default)]
    pub username: String,
    // 密码
    #[serde(default)]
    pub password: String,
    // 姓名
    #[serde(default)]
    pub name: String,
    // 性别（0-女，1-男 默认0）
    #[serde(default)]
    pub sex: u8,
    // 生日
    #[serde(default)]
    pub brithday: String,
    // 状态（0-启用，1-停用 默认0）
    #[serde(default)]
    pub status: u8,
}

#[allow(dead_code)]
impl User {
    pub fn new(id: String, username: String, password: String, name: String, sex: u8, brithday: String, status: u8) -> User {
        User {
            id,
            username,
            password,
            name,
            sex,
            brithday,
            status,
        }
    }
}