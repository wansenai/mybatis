///
/// 在Form和Into 是内在联系，而这实际上是其实现的一部分，如过能把类型A转为类型B
/// 那么对应类型B也能转换成类型A
///


///
/// From 允许一个类型定义如何从另一个类型创建自己，
/// 因此提供了一种非常简单机制来在多个类型直接进行转换
/// 标准库中有许多此特性的实现
/// 下面的例子可以将 a str 转换成 String类型
///

use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number{
    value : i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self{
        Number {value : item}
    }
}

#[test]
fn form_main(){
    let var_one = "test";
    let var_two = String::from(var_one);
    println!("var_two 转换的值: {}", var_two);

    let num = Number::from(60);
    println!("num Form类型创建: {:?}", num);
}





///
/// Into的特性是Form特性的相反，也就是如果已经使用了Form类型实现了trait，Into将在必要的时候调用它
/// 使用Into特性通常是需要指定要转换为的类型，因为编译器在大多数情况下无法确认这一点
///

#[derive(Debug)]
#[allow(dead_code)]
struct Text{
    value : String,
}

impl From<String> for Text {
    fn from(index: String) -> Self{
        Text {value : index}
    }
}


#[test]
fn into_main(){
    let var_five = String::from("test var_five");
    let text : Text = var_five.into();
    println!("var_five into: {:?}", text);
}