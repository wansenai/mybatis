pub mod nucleic_institution_service;
pub mod nucleic_registe_service;
pub mod nucleic_result_service;
pub mod user_service;

use crate::domain::nucleic_institution::InstitutionObject;

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

// 核酸机构服务
pub trait NucleicInstitutionService {

    /**
     * 注册核酸机构
     */
    fn insert_nucleic_institution(&self) -> bool;

    /**
     * 修改核酸机构
     */
    fn update_nucleic_institution(&self) -> bool;

    /**
     * 根据地区查询核酸机构
     */
    fn query_nucleic_institution_byregion(region: &str) -> Vec<InstitutionObject>;

    /**
     * 根据机构名称查询核酸机构
     */
    fn query_nucleic_institution_byname(name: &str) -> Vec<InstitutionObject>;
}

// 核酸结果服务
pub trait NucleicResultService {

    /**
     * 录入核酸结果
     */
    fn insert_nucleic_result(&self) -> bool;
}