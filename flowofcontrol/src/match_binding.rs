///
/// Match 还可以通过@符号绑定变量
///

#[test]
fn test_main(){

    fn age() -> u32{
        22
    }

    match age() {
        _i @ 1 ..=15 => println!("年龄段在1~15之间"),
        _i @ 16 ..=23 => println!("年龄段在16~23之间"),
        _ => println!("没有匹配到年龄")
    }
}

#[test]
fn test_main2(){

    fn nums() -> Option<u32> {
        Some(40)
    }

    match nums() {
        Some(_n @ 40) => println!("数字为40已匹配到"),
        Some(_n) => println!("没有任何匹配"),
        _ => (),
    }
}