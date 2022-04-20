///
/// DOC 文档
/// 表达式
/// Rust 表达式最常见的就是变量声明绑定
/// 块也是表达式，因此块中的表达式可以用作赋值，块中的最后一个表达式如果以分号;结尾，则返回()
///

fn main() {

    // 注释

    /*
     * 注释
     */

    // 变量绑定
    
        
    let var_one = 65i32;
    println!("var_one: {}",var_one);
    println!("var_one: {}",var_one + 35);
    35;
    

    // 块绑定
    let num = 5.8f64;
    let var_two = {
        let x = num * num;
        let y = x * num;

        y + x + num
    };

    let var_three = {
        2.0 * num
    };

    println!("num : {:?}", num);
    println!("var_two : {:?}", var_two);
    println!("var_three : {:?}", var_three);
}
