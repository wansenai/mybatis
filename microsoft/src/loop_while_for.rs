#[allow(dead_code)]
fn test_loop(){

    let mut nums = 5;
    loop {
        nums += 1;
        if nums == 50 {
            println!("{:?}", nums);
            break;
        }
    }
}

#[allow(dead_code)]
fn test_while(){

    let mut  result = 8;

    while result < 9 {
        result += result + 2;
    }
    println!("{:}", result);
}

#[allow(dead_code)]
fn test_for(){

    let user = vec!["zhaowei", "zhangsan", "wangwu"];

    for item in user.iter() {
        println!("名字 : {:?}", item);
    }
}

#[test]
fn main(){
    test_loop();
    test_while();
    test_for();
}