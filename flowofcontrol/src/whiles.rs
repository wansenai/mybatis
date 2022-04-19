///
/// while关键字也是用于循环，而第一个条件为true的时候才允许循环
///

#[test]
fn main(){
    
    let mut number = 3;

    while number <= 100 {

        if number % 5 == 0{
            println!("n % 5  = 0的值: {}", number);
        } else if number % 3 == 0 {
            println!("n % 3  = 0的值: {}", number);
        } else {
            println!("其他的值 {}", number);
        }
        number += 1;
    }
}