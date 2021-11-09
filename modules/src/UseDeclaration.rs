///
/// 使用use 关键字
/// use可以把完整的路径绑定到新的变量上，以便方便访问
///

use deeply::nested::function as two_funtion;

mod deeply{
    pub mod nested {
        pub fn function() {
            println!("内部调用方法");
        }
    }
}

fn function(){
    println!("调用 function()函数")
}


#[test]
fn main(){
    two_funtion();

    println!("~~~~~~~~~~~~~~");
    {
        use deeply::nested::function;
        function();
        println!("块中的函数")
    }
    function();
}

