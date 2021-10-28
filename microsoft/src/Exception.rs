enum Options<T>{
    None, // 值不存在
    Some(T), // 值存在
}

fn getCar(){

    let mut more_car = Vec::new();
    more_car.push("奔驰");
    more_car.push("奥迪");
    more_car.push("玛莎拉蒂");

    println!("{}", more_car[2]);

    // 报错 越界
    // println!("{}", more_car[999]);
}

fn match_test(){
    let name = vec!["zh","en","jp"];
    for &index in [0, 2, 9].iter(){
        match name.get(index) {
            Some(&"en") => println!("some"),
            Some(item) => println!("{}", item),
            None => println!("没有任何值"),
        }
    }
}

fn match_test_two(){
    let number : Option<u8> = Some(7);

    match number {
        Some(7) => println!("幸运数字"),
        _ => {},
    }

    if let Some(7) = number {
        println!("第二种方法 幸运数字")
    }
}

fn unwrap_test(){
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    let empty_gift : Option<&str> = None;
    // 报错 因为方法说none 代表没有值 如果直接用unwrap访问内部值 将报错
    // assert_eq!(empty_gift.unwrap(), "candy");
}

fn expect_test(){
    // expect 和 unwrap 方法相同，但他提供2个参数，第二个参数可以自定义panic
    let empty_user : Option<u32> = None;
   // assert_eq!(empty_user.expect("内部数据为null"), 1);
}

fn unwrap_or_test(){
    // 一般不建议调用unwrap() 方法， 调用unwrap_or()方法可以避免程序崩溃
    assert_eq!(None.unwrap_or("cat"), "cat2");
}

#[test]
fn main(){
    getCar();
    match_test();
    match_test_two();
    // test unwrap
    unwrap_test();
    // test expect
    expect_test();
    // test unwrap_or
    unwrap_or_test();
}