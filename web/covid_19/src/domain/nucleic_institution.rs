#[derive(Clone, Debug, PartialEq)]
pub struct InstitutionObject {
    // 雪花id
    pub id: String,
    // 核酸结果（0-阴性 1-阳性 2-无效）
    pub result_type: i32,
    // 检测机构id
    pub institution_id: String,
    // 核酸登记id
    pub registe_id: String,
    // 创建时间
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