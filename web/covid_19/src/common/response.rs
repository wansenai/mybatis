use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleResponse {
    pub code: i32,
    pub msg: String,
}

#[allow(dead_code)]
impl <T> DataResponse<T> {
    pub fn new(code: i32, msg: String, data: T) -> DataResponse<T> {
        DataResponse {
            code,
            msg,
            data,
        }
    }
}