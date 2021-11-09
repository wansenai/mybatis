///
/// Rust不提供隐式类型之前的强制转换，可以用 as 关键字进行显式类型转换
///
///

// 不让来自强制转换有警告
// #![allow(overflowing_literals)]

#[test]
fn main(){
    let decimal = 17_165.551_f32;

    // 语法报错 不能进行强制转换
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let chars = integer as char;

    // 编译报错 char类型无法转换float类型
    // let chars = chars as f32;

    println!("转换 {} -> {} -> {}",decimal, integer, chars);

    // 将任何类型强制转化无符号类型T的时候
    // T::MAX + 1 直到适合新的类型
    println!("1000 u16 类型 : {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 编译报错在rust下 前8个最低有效位被保留
    // println!("1000 u8 类型 : {}", 1000 as u8);

    // -1 + 256 = 255 输出255
    println!("-1 u8 类型 : {}", -1i8 as u8);

    println!("1000 除以 256 : {}", 1000 % 256);

    // 当强制转换为有符号类型的时候，结果跟位换算一样
    // 首先强制转换为相对应的无符号类型
    // 若该值为-1 ， 则该值为负数

    println!("128 i16 类型 : {}", 128 as i16);

    // 编译报错 128 as u8 -> 128
    // println!("128 i8 类型 : {}", 128 as i8);
    // println!("1000 u8 类型 : {}", 1000 as u8);
    // println!("232 i8 类型 : {}", 232 as i8)

    // 从rust1.45版本开始 as关键字在从float转换为int的时候执行 *saturating cast*
    // 如果浮点值超过上限或小于下限，则返回值将等于被交叉边界
    // 300.0等于255

    println!("300 u8 类型 : {}", 300.0_f32 as u8);
    // -100 等于0
    println!("-100.0 u8 类型 : {}", -100.0_f32 as u8);

    println!("nan u8 类型 : {}", f32::NAN as u8);

    //不安全的转换办法
    unsafe{

        println!("300.0 : {}:", 300.0_f32.to_int_unchecked::<u8>());

        println!("-100.0 : {}", (-100.0_f32).to_int_unchecked::<u8>());

        println!("nan : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}