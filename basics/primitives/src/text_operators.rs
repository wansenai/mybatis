// 文字和运算符

#[test]
fn test(){
    println!("1 + 2 = {}", 1u32+2);

    println!("1 - 2 = {}", 1i32-2);

    // 返回false
    println!("true and false is {}", true && false);
    // 返回true
    println!("true or false is {}", true || false);
    // 返回false
    println!("not true is {}", !true);


    // 位运算
    println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 or 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 xor 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线可以提高代码可读性
    println!("100万:{}", 1_000_000u32);
}
