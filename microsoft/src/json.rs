trait AsJson {
    fn as_json(&self) -> String;
}

fn send_to_json(value: &impl AsJson){
    println!("-> {}", value.as_json())
}

fn send_data_as_json<T: AsJson>(value : &T){
}

struct Cat{
    name : String,
    color : String,
    age : u8,
}

struct Dog{
    name : String,
    age : u8,
    hobby : String,
}

impl AsJson for Cat{
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "Cat", "name": "{}", "color": "{}", "age": "{}" }}"#,
            self.name, self.color, self.color
        )
    }
}


impl AsJson for Dog{
    fn as_json(&self) -> String {
        format!(
            r#"{{"type": "Dog", "name": "{}", age: "{}", "hobby":"{}" }}"#,
            self.name, self.age, self.hobby
        )
    }
}

#[test]
fn main(){
    let Cindy = Cat { name: "Cindy".to_string(), color: "white".to_string(), age: 1};
    send_to_json(&Cindy);
}


struct Container<T> {
    value: T,
}

impl <T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

#[test]
fn main_test() {
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}