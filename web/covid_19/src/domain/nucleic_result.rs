#[derive(Clone, Debug)]
pub struct resultObject {
    /// 核酸类型（0-单人单管 1-一户一管 2-混采）
    pub nucleic_type: i32,
    /// 姓名
    pub user_name: String,
    // 住址
    pub user_address: String,
    // 电话
    pub user_phone: String,
}

impl resultObject {
    pub fn new(nucleic_type: i32, user_name: String, user_address: String, user_phone: String) -> resultObject {
        resultObject {
            nucleic_type,
            user_name,
            user_address,
            user_phone,
        }
    }
}