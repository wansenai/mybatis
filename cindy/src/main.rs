use std::fs;

fn main(){
    println!("test zow");

    let url = "https://www.alfaright.com/";
    let output = "rust.md";

    println!("Fetching url :{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown.......");
    let md = html2md::parse_html(&body);

    fs::write(&output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output)
}

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

#[test]
fn test_other(){
    println!("apply square: {}", apply(2, square));

    println!("apply cube: {}", apply(3, cube))
}


fn pi() -> f64 {
    3.1415926
}

fn not_pi() -> f64 {
    3.1415926
}

#[test]
fn test_pi() {
    let is_pi = pi();
    let is_unit1 = not_pi();

    /// 如果最后一个表达式后面添加了; 隐含其返回值unit
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
}
