///
/// super 和 self关键字
///
#[allow(dead_code)]
fn function() {
    println!("调用 function() 函数")
}

#[allow(dead_code)]
mod one_mod {
    pub fn one_m_function() {
        println!("调用 one_m_function() 函数")
    }
}

#[allow(dead_code)]
mod two_mod {
    fn two_m_function() {
        println!("调用 two_m_function() 函数")
    }

    mod cpp {
        pub fn get_cpp_file(){
            println!("获取Cpp文件")
        }
    }

    pub fn install_all_method(){
        println!("装载所有方法");

        self::two_m_function();
        self::cpp::get_cpp_file();

        super::function();
        super::one_mod::one_m_function();
    }
}

#[test]
fn main(){
    two_mod::install_all_method();
}