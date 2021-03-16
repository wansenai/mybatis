// Display 打印比Debug打印看起来更简单 但是Display 不支持打印fmt::Binary

use std::fmt;
#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct MinMax(i64, i64);

#[derive(Debug)]
struct test{
    a: f64,
    b: f64,
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Display for test{
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result{
        write!(f, "a: {}, b: {}", self.a, self.b)
    }
}

#[test]
fn main(){
    let minmax = MinMax(66, 77);
    println!("打印结果:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let testA = MinMax(555, -666);
    let testB = MinMax(-8, 9);

    println!("测试数据A:{a}~~~~数据B:{b}", a = testA, b = testB);

    let point = test{a: 6.5, b:7.3};
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}