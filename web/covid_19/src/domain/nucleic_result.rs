use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResultObject {
    // 核酸类型（0-单人单管 1-一户一管 2-混采）
    #[serde(default)]
    pub nucleic_type: i32,
    // 姓名
    #[serde(default)]
    pub user_name: String,
    // 住址
    #[serde(default)]
    pub user_address: String,
    // 电话
    #[serde(default)]
    pub user_phone: String,
}

#[allow(dead_code)]
impl ResultObject {
    pub fn new(nucleic_type: i32, user_name: String, user_address: String, user_phone: String) -> ResultObject {
        ResultObject {
            nucleic_type,
            user_name,
            user_address,
            user_phone,
        }
    }
}