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

///
/// 
/// # example
/// 
/// ```
/// let nucleic_type = 0;
/// let user_name = String::from("赵伟");
/// let user_address = String::from("上海市静安区虬江路1488号14号");
/// let user_phone = String::from("16616616161")
/// 
/// resultObject::new(nucleic_type, 
///                 user_name, user_address, 
///                 user_phone);
/// 
/// ```
/// 
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