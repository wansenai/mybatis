///
/// 从loop循环中返回
/// 如果循环需要返回一个值，需要传递给其他代码，将其放在loop表达式break后面
///

#[test]
fn main(){

    let mut count =  0.0f32;

    let result = loop {
        count += 0.5;

        if count == 7.5 {
            break count * 2.0;
        }
    };
    
    println!("{}", count);
    println!("{}", result);
}