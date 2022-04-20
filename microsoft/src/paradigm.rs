#[allow(dead_code)]
struct Point<T>{
    x : T,
    y : T
}

#[allow(dead_code)]
struct Print<T, U>{
    j : T,
    k : U
}

#[test]
#[allow(dead_code)]
fn get_point(){
    let _var1 = Point{ x: "json", y: "text" };
    let _var2 = Point{ x: 32, y: 64 };
}

#[test]
fn get_print(){
    let _var3 = Print{ j: true, k: "test" };
}

trait Area {
    fn get_info(&self) -> f64;
}

struct Today {
    todolist : f64
}

impl Area for Today{
    fn get_info(&self) -> f64 {
        use std::f64::consts::PI;
        return PI * self.todolist.powf(2.0);
    }
}

#[derive(Debug, PartialEq)]
struct NewPoint{
    x : i32,
    y : i32
}

#[test]
fn eq_x_y(){
    let var1 = NewPoint{
        x : 3,
        y : 5
    };

    let var2 = NewPoint{
        x : 8,
        y : -5
    };
    println!("{:}", var1.x);
    println!("{:?}", var2.y);
    if var1 == var2 {
        println!("eq");
    }else {
        println!("not eq");
    }
}
