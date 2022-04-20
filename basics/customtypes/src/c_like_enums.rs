// rust enum 也可以用于C类 enum

// 注释掉未引用代码 警告
#![allow(dead_code)]

// 隐式枚举（从0开始）
enum Number{
    One,
    Two,
    Three,
}

// 显式枚举
enum Color{
    Red  = 0xff0000,
    Gree = 0x00ff00,
    Blue = 0x4a86e8,
}


#[test]
fn main(){

    // enums可以转化为整数
    println!("One {:?}", Number::One as i32);
    println!("Two {:}",  Number::Two as i32);
    println!("Three {}", Number::Two as i32);

    println!("#{:06x}", Color::Red as i32);
    println!("#{:06x}", Color::Blue as i32);
}