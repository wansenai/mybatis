pub mod nucleic_institution_service;
pub mod nucleic_registe_service;
pub mod user_service;

// 核酸check
pub trait Nucleic {
    fn query_map(&self) -> i32;

    fn test(&self) -> i32;
}

// 用户服务
pub trait UserService {
    /** 
     * 新增用户
     */
    fn insert_user(&self) -> bool;

    /** 
     * 用户登陆
     */
    fn user_login(&self, username: &str, password: &str) -> bool;
}

// 核酸服务
pub trait NucleicService {

    /** 
     * 核酸登记
     */
    fn nucleic_registe(&self) -> bool;
}
