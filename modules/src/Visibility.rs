///
/// module 的 public private修饰
///
#[allow(dead_code)]
mod test_mod {
    
    fn private_function(){
        println!("私有函数")
    }

    pub fn get_number(num: u32) -> u32 {
        let num_two = 2 + num;
        return num_two;
    }

    // 同一模块可以访问私有函数
    pub fn get_function(){
        println!("获取函数");
        private_function();
    }

    // 模块嵌套
    pub mod nested {

        pub fn function_child_module(){
            println!("内嵌模块里的公用方法");
        }

        #[allow(dead_code)]
        fn function_private_child_module(){
            println!("~~~~~~~~:)");
        }

        // 使用 pub (self) fn 声明函数内可见
        pub (self) fn public_function_in_self(){
            println!("pub (self) fn 函数")
        }

        // 使用 pub (super) fn 声明 父模块
        pub (super) fn public_function_in_super(){
            println!("pub (super) fn 函数")
        }
    }

    pub fn public_function_in_test_mod(){

        nested::public_function_in_super()
    }

    pub (crate) fn public_crate_function(){
        println!("调用 public_crate_function() 函数")
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("私有内嵌模块的方法")
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("restricted_function 函数");
        }
    }
}


#[test]
fn main(){
    fn function(){
        println!("没有模块的函数")
    }

    function();

    test_mod::get_number(5);
    test_mod::nested::function_child_module();

    test_mod::public_crate_function();
}