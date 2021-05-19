pub mod inaccessible;
pub mod nested;

pub fn mod_function(){
    println!("调用file文件夹下的mod_function")
}

fn private_function(){
    println!("mod 私有方法")
}