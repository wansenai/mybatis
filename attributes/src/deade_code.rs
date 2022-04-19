///
/// deade_code 用户警告未使用的功能
///

#[allow(dead_code)]
fn used_function(){
    println!("使用该函数");
}

#[allow(dead_code)]
fn unused_function(){

}

#[test]
fn main(){
    used_function();
}