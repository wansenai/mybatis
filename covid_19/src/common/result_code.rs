use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ResultCode {

    Custom(u32, String),

    Success(u32, String),

    Fail(u32, String),

    ParamNotValid(u32, String),

    SystemExecutionError(u32, String),

    NoDataFound(u32, String),
}

#[allow(dead_code)]
impl ResultCode {

    pub fn custom(code: u32, message: String) -> ResultCode {
        ResultCode::Custom(code, message)
    }
    
    pub fn success() -> ResultCode {
        ResultCode::Success(200, String::from("成功"))
    }

    pub fn fail() -> ResultCode {
        ResultCode::Fail(500, String::from("失败"))
    }

    pub fn param_not_valid() -> ResultCode {
        ResultCode::ParamNotValid(1001, String::from("参数无效"))
    }

    pub fn system_execution_error() -> ResultCode {
        ResultCode::SystemExecutionError(3002, String::from("系统内部错误"))
    }

    pub fn no_data_found() -> ResultCode {
        ResultCode::NoDataFound(4004, String::from("没有查询到对应的数据"))
    }

    pub fn get_code(data: &ResultCode) -> u32 {
        match data {
            ResultCode::Custom(code, _)                     => *code,
            ResultCode::Success(code, _)                    => *code,
            ResultCode::Fail(code, _)                       => *code,
            ResultCode::ParamNotValid(code, _)              => *code,
            ResultCode::SystemExecutionError(code, _)       => *code,
            ResultCode::NoDataFound(code, _)                => *code,
        }
    }

    pub fn get_message(data: &ResultCode) -> &str {
        match data {
            ResultCode::Custom(_, message)                      => message,
            ResultCode::Success(_, message)                     => message,
            ResultCode::Fail(_, message)                        => message,
            ResultCode::ParamNotValid(_, message)               => message,
            ResultCode::SystemExecutionError(_, message)        => message,
            ResultCode::NoDataFound(_, message)                 => message,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_success() {

        println!("{:?}", ResultCode::get_code(&ResultCode::success()));
        println!("{:?}", ResultCode::get_code(&ResultCode::fail()));
        println!("{:?}", ResultCode::get_code(&ResultCode::param_not_valid()));
        println!("{:?}", ResultCode::get_code(&ResultCode::system_execution_error()));
        println!("{:?}", ResultCode::get_code(&ResultCode::no_data_found()));
    }
}