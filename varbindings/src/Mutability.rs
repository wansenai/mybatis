///
/// 默认情况下声明了一个变量 变量绑定是不可变的
/// 但是可以使用mut 修饰变量，mut可以改变
///


#[test]
fn main(){

    let not_var = 1;

    assert_eq!(not_var,1);

    let mut yes_var = 2;

    println!("变量可变之前: {}", yes_var);

    yes_var += 1;
    println!("变量可变之后: {}", yes_var);

    // 会提示语法错误，因为变量绑定是不可变的，除非用mut修饰符进行修饰
    // not_var += 1;
}