fn say(){
    print!("say hello")
}

#[test]
fn goodbye(){
    say();
    print!(" goodbye");
}

fn test(message : &str){
    println!("\n{}", message);
}

#[test]
fn test_two(){
    let var_one = "love you";
    test(var_one);
}

fn test_three(num : i32) -> i32 {
    num * 5
}

#[test]
fn test_flour(){
    let var_two = 5;
    println!("value: {}", test_three(5));
}