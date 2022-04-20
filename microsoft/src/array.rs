#[allow(dead_code)]
fn create_array(){
    // 没有定义长度的数组
    let animals = ["Tiger", "Cat", "Dog", "Lion"];

    let var_cat = animals[1];

    // let var_cat = animals[4]; 报错 数组越界
    println!("{}", var_cat);
}

#[allow(dead_code)]
fn create_vector(){
    let three_nums = vec![20, 3, 17];
    println!("{:?}", three_nums);

    let zeros = vec![0; 6];
    println!("{:?}", zeros);
    println!("{:?}", zeros);
}

#[allow(dead_code)]
fn custom_array(){
    let mut user = Vec::new();

    user.push("zhaowei");
    user.push("wanglei");
    user.push("test");
    println!("{:?}", user);

    // user.push(1) 报错 因为第一个元素push的时候是&str 类型

    // delete 末尾元素 pop()
    println!("删除元素{:?}", user.pop());
    println!("已经删除的元素{:?}", user);

    let mut numbers = Vec::new();
    numbers.push(12);
    numbers.push(5);
    numbers.push(25);
    println!("集合:{:?},第二个元素是:{:}", numbers, numbers[1]);

    numbers[1] = numbers[1] + 5;
    println!("{:?}", numbers[1]);

}

#[test]
fn main(){
    create_array();
    create_vector();
    custom_array();
}
