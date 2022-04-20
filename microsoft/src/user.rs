#[allow(dead_code)]
struct User{
    name : String,
    sex : char,
    age : i32,
}

#[allow(dead_code)]
fn create_user(var_name: String, var_age: i32, var_sex: char) -> User{
    User{
        name: var_name,
        age: var_age,
        sex: var_sex,
    }
}

#[test]
fn main(){
    let user = create_user("wang".to_string(), 23, '男');
    println!("{},{},{}",user.name,user.age,user.sex);
}
