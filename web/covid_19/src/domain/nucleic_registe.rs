#[derive(Clone, Debug, PartialEq)]
pub struct NucleicRegiste {
    // 主键 雪花id
    id: String,
    // 核酸类型（0-单人单管 1-一户一管 2-混采）
    nucleic_type: u8,
    // 用户姓名
    name: String,
    // 用户居住地址
    address: String,
    // 用户电话
    phone: String,
    // 创建时间
    create_time: String,
    // 更新时间
    update_time: String,
}

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