///
/// 闭包 可以捕获封闭环境的变量和函数 例如 |val| val + x  捕获x变量的闭包
/// 在输入变量时使用|| 而不是 ()
/// 单个表达式可以选择 ({})
///

#[test]
fn main(){

    fn function_test(var : i32) -> i32 {
        var + 2
    }

    // 闭包是匿名的，注释与函数注释相同，就像包括方法体一样{}
    let closure_annotated = |i : i32| -> i32 { i * 2 };
    let closure_inferred = |i   | i * 4 ;

    let i = 1;
    println!("通用函数 : {}", function_test(i));
    println!("annotated 闭包 : {}", closure_annotated(i));
    println!("inferred 闭包 : {}", closure_inferred(i));

    let var =  || 1;
    println!("普通var : {}", var());
}