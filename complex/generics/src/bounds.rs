///
/// Bounds 当使用泛型的时候，有的参数需要使用Traits界限来规定实现什么功能
/// 比如使用trait Display打印，需要T被Display; 也就是T必须执行Display
///
use std::fmt::Display;
use std::fmt::Debug;


// 定义函数实现trait Display
#[allow(dead_code)]
fn printer<T: Display>(t: T){
    println!("{}", t);
}

#[allow(dead_code)]
struct S<T: Display>(T);


#[derive(Debug)]
struct Rectangle { length : f64, height : f64 }

#[derive(Debug)]
#[allow(dead_code)]
struct Triangle { length : f64, height : f64 }

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[allow(dead_code)]
fn print_debug<T: Debug>(t: &T){
    println!("{:?}", t);
}

#[allow(dead_code)]
fn area<T: HasArea>(t: &T) -> f64{
    t.area()
}

#[test]
fn main(){
    let rectangle = Rectangle {length : 3.0, height: 2.2};
    let _triangle = Triangle {length : 1.7, height: 5.8};
    print_debug(&rectangle);

    println!("面积：{}", area(&rectangle));
}
