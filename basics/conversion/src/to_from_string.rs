///
/// 字符串转换
/// 将任何类型转换为一个String就像实现ToString一样，不因该这样写
/// 应该实现fmt::Display自动字符串的特性
///

use std::fmt;

struct Module{
    var_one : i32
}

impl fmt::Display for Module{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "测试数据: {}", self.var_one)
    }
}

#[test]
fn main(){
    let var = Module{ var_one : 6};
    println!("{}", var.to_string())
}



///
/// 解析字符串
/// 将经常使用的数字转换为字符串是最常见的。只需要将FromStr 特征字符串实现为该类型，它将把字符串转为指定类型
/// 只需要将FromStr 字符串实现该类型，它将把字符串转为指定类型
///

#[test]
fn test(){
    let var_two : i32 = "5".parse().unwrap();
    let var_three  = "10".parse::<i32>().unwrap();

    let sum = var_two + var_three;
    println!("sum = {} ", sum);
}