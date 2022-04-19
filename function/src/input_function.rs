///
/// 输入功能
/// 由于闭包可以用作参数，闭包都可以作为参数传递
///

// 定义一个接受泛型F的参数函数
// 以 Fn 为界 并调用它
#[allow(dead_code)]
fn call_fun<F: Fn()>(f : F) {
    f();
}

// 定义满足Fn绑定的包装函数
#[allow(dead_code)]
fn function_test(){
    println!("test function")
}

#[test]
fn main(){
    let var_one = || println!("test");
    call_fun(var_one);
    call_fun(function_test);
}