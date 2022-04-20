///
/// 可以通过进行变量添加类型后缀，对该变量进行类型修饰，例如要制定25为i32类型，可以写成25i32
/// 如果不带后缀或者不指定格式的话，编译器将使用i32或者f64进行修饰数字
///


#[test]
fn main(){
    let var_one = 25i32;
    let var_two = 5.6f32;
    let var_three = "test";
    let var_flor : f64 = 55.6;

    println!("var_one : {}", var_one);
    println!("var_two : {}", var_two);
    println!("var_three : {}", var_three);
    println!("var_flor : {:?}", var_flor);

    // size_of_val 是一个函数，可以将代码拆分成 模块的逻辑单元，size_of_val需要mem定义，而mem是需要std crate创建出来的
    println!("size of var_one in bytes : {}", std::mem::size_of_val(&var_one));
    println!("size of var_two in bytes : {}", std::mem::size_of_val(&var_two));
    println!("size of var_three in bytes : {}", std::mem::size_of_val(var_three));
    println!("size of var_flor in bytes : {}", std::mem::size_of_val(&var_flor));
}