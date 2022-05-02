use serde::{Deserialize, Serialize};
use super::result_code::ResultCode;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Result<T> {
    pub code: u32, 
    pub msg: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[allow(dead_code)]
impl <T> Result<T> {
    pub fn new(code: u32, msg: String, data: T) -> Result<T> {
        Result {
            code,
            msg,
            data : Some(data),
        }
    }

    pub fn success_data(data: T) -> Result<T> {
        
        let success = ResultCode::success();
        
        let result = Result::new(
            ResultCode::get_code(&success), 
            String::from(ResultCode::get_message(&success)),
            data,
        );

        result
    }
}

pub trait ResultDefault {
    fn new(code: u32, msg: String) -> Result<ResultCode>;

    fn success() -> Result<ResultCode>;

    fn fail() -> Result<ResultCode>;
}

impl ResultDefault for Result<ResultCode> {
    fn new(code: u32, msg: String) -> Result<ResultCode> {
        Result {
            code: code, 
            msg: msg,
            data: None,
        }
    }

    fn success() -> Result<ResultCode> {
        let success = ResultCode::success();

        Result {
            code: ResultCode::get_code(&success),
            msg:  String::from(ResultCode::get_message(&success)),
            data: None,
        }
    }

    fn fail() -> Result<ResultCode> {
        let fail = ResultCode::fail();
        
        Result {
            code: ResultCode::get_code(&fail),
            msg:  String::from(ResultCode::get_message(&fail)),
            data: None,
        }
    }
}