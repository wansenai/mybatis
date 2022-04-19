mod function_test;
mod user;
mod array;
mod condition;
mod hash_map;
mod loop_while_for;
mod error;
mod exception;
mod option_test;
mod result;
mod all;
mod paradigm;
mod json;
mod mod_load;
mod util_test;
mod doc_test;
mod pizza;

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
    let user = Object{name: String::from("zhaowei"), level:100, remote:true};
    let user_two = Object{name: "zhangsan".to_string(), level:100, remote:true};
    println!("{}: {}, {}, {}", user.name, user.level, user_two.name, user_two.remote);

    #[allow(dead_code)]
    enum EnuVet{
        VarOne,
        VarTwo(String, char),
        VarThree{x: i32, y: i64}
    }

    #[allow(dead_code)]
    struct KeyPress(String, char);
    #[allow(dead_code)]
    struct MouseClick{x: i64, y: i64}

    #[allow(dead_code)]
    enum Test{
        VarOne(bool),
        VarTwo(KeyPress),
        VarThree(MouseClick)
    }

    let _v_test = Test::VarThree{ 0: MouseClick {x : 12, y : 15}};
    let v_test_two = KeyPress("zhaowei".to_string(), 'y');
    println!("{}: {}", v_test_two.0, v_test_two.1);

}
