enum Option<T>{
    None, // 值不存在
    Some(T), // 值存在
}

fn getCar(){

    let mut more_car = Vec::new();
    more_car.push("奔驰");
    more_car.push("奥迪");
    more_car.push("玛莎拉蒂");

    println!("{}", more_car[2]);

    // 报错 越界
    // println!("{}", more_car[999]);
}

fn match_test(){
    let name = vec!["zh","en","jp"];
    for &index in [0, 2, 9].iter(){
        match name.get(index) {
            Some(&"en") => println!("some"),
            Some(item) => print!("{}", item),
            None => print!("没有任何值"),
        }
    }
}



#[test]
fn main(){
    getCar();
    match_test();
}
