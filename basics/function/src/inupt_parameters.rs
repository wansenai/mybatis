///
/// 作为输入参数的 函数
/// Rust 在没有任何类型的注释情况下即使捕获变量的方法，当将闭包作为输入参数的时候
/// 必须使用少数几个对闭包完整类型进行注释traits
/// Fn  :   闭包通过引用(&T)捕获
/// FnMut   :   闭包通过可变变量引用(&mut T)捕获
/// FnOnce  :   闭包按值(T)捕获
///
#[allow(dead_code)]
fn apply<F>(f : F)  where
    F : FnOnce() {
    f();
}

#[allow(dead_code)]
fn apply_to_2<F>(f : F) -> i32 where
    F : Fn(i32) -> i32 {
    f(2)
}

#[test]
fn main(){
    use std::mem;

    let var_one = "hello";
    let mut var_two = "James".to_owned();

    let diary = || {
        println!("var_one : {}", var_one);
        var_two.push_str("~~~");
        println!("var_two : {}", var_two);
        mem::drop(var_two);
    };

    apply(diary);

    let var_three = |x| 2 * x;
    println!("apply_to_2函数计算的值 : {}", apply_to_2(var_three));
}