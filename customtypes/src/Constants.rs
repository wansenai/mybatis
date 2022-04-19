///
/// Rust有两种创建常量的方式，一种是const，另外一种是static
/// 两种都可以在任何范围内（全局范围内）创建常量，但都需要显式类型批注
///
/// const  : 不可更改值
/// static : 一个可能可变的变量，具有静态生命周期，类似java访问不必指定，但访问或者修改静态变量是不安全的
///

const CODE: i32 = 200;

#[allow(dead_code)]
pub fn is_bug(n: i32) -> bool{
    n > CODE
}


#[test]
fn test(){
    static STATUS: &str = "enabled";
    let n = 201;

    println!("打印const常量 {}", CODE);
    println!("打印static常量 {:?}", STATUS);
    println!("比较两个值{} is {}", n, if is_bug(n) {"大"} else {"小"});

    // 报错 因为无法static 变量
    // STATUS = "aaa";
}

