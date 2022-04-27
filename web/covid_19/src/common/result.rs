use serde::{Deserialize, Serialize};
use super::result_code::ResultCode;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResultData<T> {
    pub code: u32, 
    pub msg: String,
    pub data: T,
}

#[allow(dead_code)]
impl <T> ResultData<T> {
    pub fn new(code: u32, msg: String, data: T) -> ResultData<T> {
        ResultData {
            code,
            msg,
            data,
        }
    }

    pub fn success_data(data: T) -> ResultData<T> {
        
        let success = ResultCode::success();
        
        let result = ResultData::new(
            ResultCode::get_code(&success), 
        ResultCode::get_message(&success).to_string(), 
            data,
        );

        result
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResultDefault {
    pub code: u32, 
    pub msg: String,
}

pub trait Result {
    fn new(code: u32, msg: String) -> ResultDefault;

    fn success() -> ResultDefault;

    fn fail() -> ResultDefault;
}

impl Result for ResultDefault {
    fn new(code: u32, msg: String) -> ResultDefault {
        ResultDefault {
            code: code, 
            msg: msg,
        }
    }

    fn success() -> ResultDefault {
        let success = ResultCode::success();

        ResultDefault {
            code: ResultCode::get_code(&success),
            msg:  ResultCode::get_message(&success).to_string(),
        }
    }

    fn fail() -> ResultDefault {
        let fail = ResultCode::fail();
        
        ResultDefault {
            code: ResultCode::get_code(&fail),
            msg:  ResultCode::get_message(&fail).to_string(),
        }
    }
}