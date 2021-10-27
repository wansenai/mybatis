mod FunctionTest;
mod User;
mod Array;
mod Condition;

use std::borrow::Borrow;

fn main() {
    println!("Hello, world!");
    // todo!("this code not use");
    let a_vrb = 50;
    println!("{}", a_vrb);

    let mut a_vrc = 25;
    a_vrc = a_vrc - 5;
    println!("{:}", a_vrc);

    let number : u32 = 6;
    println!("{}", number);

    let number_two : f64 = 6.54;
    println!("{}", number_two);

    let smiley_face = '😃';
    println!("{}", smiley_face);

    // let var_one: str  = "test";  error
    let var_two : &str = "test2";
    println!("{}", var_two);

    let tuple = (1, "test", 0.35);
    println!("{}:{}:{}", tuple.0, tuple.1, tuple.2);

    struct Object{name: String, level: i64, remote: bool}
    struct ObjectT(String, char, i16);
    struct Unit;


    let user = Object{name: String::from("zhaowei"), level:100, remote:true};
    let user_two = Object{name: "zhangsan".to_string(), level:100, remote:true};
    println!("{}: {}", user.name, user_two.name);

    enum EnuVet{
        VarOne,
        VarTwo(String, char),
        VarThree{x: i32, y: i64}
    }

    struct KeyPress(String, char);
    struct MouseClick{x: i64, y: i64}
    enum Test{
        VarOne(bool),
        VarTwo(KeyPress),
        VarThree(MouseClick)
    }

    let v_test = Test::VarThree{ 0: MouseClick {x : 12, y : 15}};
    let v_test_two = KeyPress("zhaowei".to_string(), 'y');
    println!("{}: {}", v_test_two.0, v_test_two.1);

}
