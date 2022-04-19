#[allow(dead_code)]
fn test_if(){
    if 1 == 2 {
        println!("1 等于 2");
    }else {
        println!("1 不等于 2");
    }
}

#[allow(dead_code)]
fn test_if_one(){
    let result = true;

    let test = if result {
        "good day to you"
    }else {
        "bye"
    };
    println!("{}", test);
}

#[allow(dead_code)]
fn test_elseif(){
    let num = 50;

    if num == 0 {
        println!("0");
    }else if num > 30{
        println!("50 > 30")
    }else if num > 51{
        println!("51 > 50")
    }else{
        println!("no data")
    }
}

#[test]
fn main(){
    test_if();
    test_if_one();
    test_elseif();
}



