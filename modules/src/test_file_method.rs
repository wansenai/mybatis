#[test]
fn main() {
    use crate::file::nested;
    use crate::file::inaccessible;

    fn self_method(){
        println!("测试方法")
    }

    self_method();
    nested::get_url();
    inaccessible::get_jvmparam();
}