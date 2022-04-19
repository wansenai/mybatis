#[allow(dead_code)]
struct Person{
    first : String,
    middle : Option<String>,
    last : String,
}

#[allow(dead_code)]
fn create_full_name(person : &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    if let Some(middle) = &person.middle {
        full_name.push_str(&middle);
        full_name.push_str(" ");
    }

    full_name.push_str(&person.last);
    full_name
}

#[test]
fn main(){
    let zhaowei = Person {
        first : "zhao".to_string(),
        middle : Some("w".to_string()),
        last : "ei".to_string(),
    };
    assert_eq!(create_full_name(&zhaowei), "zhao w ei");

    let wangxiaoer = Person {
        first : String::from("wang"),
        middle : None,
        last : String::from("er")
    };
    assert_eq!(create_full_name(&wangxiaoer), "wang er");
}