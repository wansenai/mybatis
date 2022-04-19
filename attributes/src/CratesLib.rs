///
/// crate_type 属性可以用于告诉编译器该Crate是文件还是库
/// crate_name 属性可以定义Crate名称
///

// #![crate_type = "lib"]
// #![crate_name = "rary"]

#[allow(dead_code)]
fn pri_function(){
    println!("私有方法");
}

#[allow(dead_code)]
pub fn pub_function(){
    println!("公有方法");
}

#[test]
fn main(){
    pri_function();
}