use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NucleicRegiste {
    // 主键 雪花id
    pub id: String,
    // 核酸类型（0-单人单管 1-一户一管 2-混采）
    pub nucleic_type: u8,
    // 用户姓名
    pub name: String,
    // 用户居住地址
    pub address: String,
    // 用户电话
    pub phone: String,
    // 创建时间
    pub create_time: String,
    // 更新时间
    pub update_time: String,
}

#[allow(dead_code)]
impl NucleicRegiste {
    pub fn new(id: String, nucleic_type: u8, name: String, address: String, phone: String, create_time: String, update_time: String) -> NucleicRegiste {
        NucleicRegiste{
            id,
            nucleic_type,
            name,
            address,
            phone,
            create_time,
            update_time,
        }
    }
}