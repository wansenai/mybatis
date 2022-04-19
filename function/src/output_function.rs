///
/// 作为输出参数
/// Fn  :   闭包通过引用(&T)捕获
/// FnMut   :   闭包通过可变变量引用(&mut T)捕获
/// FnOnce  :   闭包按值(T)捕获
///
/// move必须使用关键字,该关键字表示所有捕获都是按值进行的
///
#[allow(dead_code)]
fn create_fn() -> impl Fn(){
    let var_text = "Fn".to_owned();
    move || println!("测试 {}", var_text)
}

#[allow(dead_code)]
fn create_fnmut() -> impl FnMut(){
    let var_text2 = "FnMut".to_owned();
    move || println!("测试 {}", var_text2)
}

#[allow(dead_code)]
fn create_fn_once() -> impl FnOnce(){
    let var_text3 = "FnOnce".to_owned();
    move || println!("测试 {}", var_text3)
}

#[test]
fn main(){
    let fn_plain = create_fn();
    let mut fnmut = create_fnmut();
    let fn_once = create_fn_once();

    fn_plain();
    fnmut();
    fn_once();
}
