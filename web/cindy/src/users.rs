struct User {
    username: String,
    password: String,
    sex: u8,
    email: String,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test_create_user(){
    let user_one = User {
        username: String::from("zhaowei"), 
        password: String::from("passw0rd"),
        sex: 23,
        email: String::from("jameszow@163.com"),
        active: true,
    };

    let mut user_two = User {
        username: String::from("test"), 
        password: String::from("passw0rd"),
        sex: 32,
    };

    let user_three = User {
        username: String::from("test2"),
        password: String::from("a123456"),
        active: user_one.active,
    };

    let user_four = User {
        username: String::from("test3"),
        password: String::from("b123456"),
        ..user_one,
    };
}

fn build_user(email: String, password: String) -> User {
    User {
        password,
        email,
        active: true,
    }
}

fn tuple_struct(){
    let color_one = Color(251, 77, 152);
    let point_one = Point(74, 82, 135);
}

fn sum_test(){
    let rectangle = Rectangle {
        width : 30,
        height : 50,
    };

    let result: u32 = cops(&rectangle);

    assert_eq!(1500, result);
}

fn cops(rectangle : &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn squart(width : u32, height : u32) -> Rectangle {
        Rectangle {width, height}
    }
}