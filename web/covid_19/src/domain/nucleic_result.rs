use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResultObject {
    // 主键id
    #[serde(default)]
    pub id: String,
    // 核酸结果（0-阴性 1-阳性 2-无效）
    #[serde(default)]
    pub nucleic_type: u8,
    // 核酸机构id
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
impl ResultObject {
    pub fn new(id: String, nucleic_type: u8, institution_id: String, registe_id: String, create_time: String) -> ResultObject {
        ResultObject {
            id,
            nucleic_type,
            institution_id,
            registe_id,
            create_time,
        }
    }
}