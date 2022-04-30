use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InstitutionObject {
    // 雪花id
    #[serde(default)]
    pub id: Option<String>,
    // 核酸机构名称
    #[serde(default)]
    pub institution_name: Option<String>,
    // 核酸机构地址
    #[serde(default)]
    pub institution_address: Option<String>,
    // 核酸机构电话
    #[serde(default)]
    pub institution_phone: Option<String>,
    // 核酸机构所属区域
    #[serde(default)]
    pub institution_region: Option<String>,
    // 创建时间
    #[serde(default)]
    pub create_time: Option<String>,
    // 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[allow(dead_code)]
impl InstitutionObject {
    pub fn new(id: String, institution_name: String, institution_address: String, institution_phone: String, 
        institution_region: String, create_time: String, update_time: String) -> InstitutionObject {
        InstitutionObject {
            id: Some(id),
            institution_name: Some(institution_name),
            institution_address: Some(institution_address),
            institution_phone: Some(institution_phone),
            institution_region: Some(institution_region),
            create_time: Some(create_time),
            update_time: Some(update_time),
        }
    }
}