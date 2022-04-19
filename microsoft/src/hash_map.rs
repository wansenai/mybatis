use std::collections::HashMap;

#[allow(dead_code)]
fn test(){
    let mut user_map : HashMap<String, String> = HashMap::new();

    user_map.insert(String::from("name"), String::from("zhaowei"));
    user_map.insert(String::from("sex"), String::from("男"));
    user_map.insert(String::from("hobby"), String::from("coding"));
    println!("{:?}", user_map);

    let name : &str = "name";
    println!("{:?}", user_map.get(name));

    let sex : &str = "sex";
    user_map.remove(sex);
    println!("{:?}", user_map.get(sex));
}

#[test]
fn main(){
    test();
}