use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NucleicResultObject {
    // 雪花id
    #[serde(default)]
    pub id: String,
    // 核酸结果（0-阴性 1-阳性 2-无效）
    #[serde(default)]
    pub result_type: i32,
    // 检测机构id
    #[serde(default)]
    pub institution_id: String,
    // 核酸登记id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registe_id: Option<String>,
    // 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[allow(dead_code)]
impl NucleicResultObject {
    pub fn new(id: String, result_type: i32, institution_id: String, registe_id: String, create_time: String) -> NucleicResultObject {
        NucleicResultObject {
            id,
            result_type,
            institution_id,
            registe_id: Some(registe_id),
            create_time:  Some(create_time),
        }
    }

    pub fn tets(id: String, result_type: i32, institution_id: String) -> NucleicResultObject {
        NucleicResultObject {
            id: id, 
            result_type: result_type,
            institution_id: institution_id,
            registe_id: None,
            create_time: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn test_ingore_param() {
        let a = RefCell::new(Rc::new(NucleicResultObject::tets(String::from("4"), 6, String::from("7"))));

        println!("{:?}", a);
    }


    #[test]
    fn a() {
      
            let x: i32;
            let printer = move |whatever: i32| { println!("whatever is: {}", whatever); };
            printer(2);
            x = 7;
            println!("x is: {}",x); // ERROR: use of possibly-uninitialized `x`
        
    }
}