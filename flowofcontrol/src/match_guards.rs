///
/// 可以添加过滤器给Match
/// 比如添加if条件语句
///

#[test]
fn main(){

    let pair = (6, -5);

    match pair {
        (x , y) if x == y => println!("两个值是相等的"),
        (x , y) if x + y == 0 => println!("两个子相加"),
        (x , y) if x - y == 11 => println!("正数"),
        _ => println!("没有匹配到"),
    }
}

#[test]
fn test_main(){

    let num : u8 = 9;

    match num {
        i if i == 0 => println!("等于0"),
        i if i == 8 => println!("等于8"),
        _ => println!("没有匹配")
    }
}