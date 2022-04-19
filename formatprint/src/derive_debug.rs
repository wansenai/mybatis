
// derive（自动创建）fmt::Debug实现, 所有std库类型也可以自动打印{:?}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
#[allow(dead_code)]
struct User<'a> {
    name : &'a str,
    age : u8
}

#[test]
fn print(){
    println!("{:?} 月份", 12);

    println!("{0:?} 喜欢吃 {1:?}", "James", "苹果");

    println!("{:?}", Structure(3));

    println!("{:?}", Deep(Structure(5)));
}

#[test]
fn user_print(){
    let name = "James";
    let age = 22;
    let user = User {name, age};
    print!("{:#?}", user);
}
