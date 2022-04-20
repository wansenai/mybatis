pub mod inaccessible;
pub mod nested;

#[allow(dead_code)]
pub fn mod_function(){
    println!("调用file文件夹下的mod_function")
}

#[allow(dead_code)]
fn private_function(){
    println!("mod 私有方法")
}