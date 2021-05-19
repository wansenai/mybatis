use crate::file::nested;
use crate::file::inaccessible;

fn self_method(){
    println!("测试方法")
}


#[test]
fn main() {
    self_method();
    nested::getUrl();
    inaccessible::getJVMParam();
}