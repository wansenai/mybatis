///
/// if-else 判断，Rust的判断不需要用括号括起来，但是每个条件后必须有个块{}
/// 并且所有分支返回相对应的类型
///

#[test]
fn main(){
    let n = 6i32;

    if n < 0 {
        println!("n的值小于0,为: {}", n);
    }else if n > 0 {
        println!("n的值大于0,为: {}", n)
    }else{
        println!("没有值")
    }

    let big_n =
        if n < 10 && n > -10{
            println!("给n*10");
            10 * n
        }else{
            println!("给n/2");
            n / 2
        };
    println!("{} -> {}", n, big_n);
}
