use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]pub struct InstitutionObject {
    // 雪花id
    #[serde(default)]
    pub id: String,
    // 核酸结果（0-阴性 1-阳性 2-无效）
    #[serde(default)]
    pub result_type: i32,
    // 检测机构id
    #[serde(default)]
    pub institution_id: String,
    // 核酸登记id
    #[serde(default)]
    pub registe_id: String,
    // 创建时间
    #[serde(default)]
    pub create_time: String,
}

#[allow(dead_code)]
impl InstitutionObject {
    pub fn new(id: String, result_type: i32, institution_id: String, registe_id: String, create_time: String) -> InstitutionObject {
        InstitutionObject {
            id,
            result_type,
            institution_id,
            registe_id,
            create_time,
        }
    }
}