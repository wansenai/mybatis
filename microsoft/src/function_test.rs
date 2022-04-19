#[test]
fn test_flour(){

    fn say(){
        print!("say hello")
    }
    
    fn test(message : &str){
        println!("\n{}", message);
    }
    
    fn test_three(num : i32) -> i32 {
        num * 5
    }

    say();
    let var_one = "love you";
    test(var_one);

    test_three(6);

    println!("value: {}", test_three(5));
}