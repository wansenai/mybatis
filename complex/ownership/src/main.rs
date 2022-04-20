///
/// rust programming language book 1 ~ 3 learing :)
/// 
fn main() {
    let x = 5;
    let x = x * 7;

    println!("{:?}", x);

    let strc : u32 = "42".parse().expect("Invalid");
    
    println!("{:?}",strc);

    let y: u8 = 255;
    println!("{}", y);

    let nums: [i32; 5] = [0,1,2,3,4];

    println!("{:?}",nums[1]);

    let tup = (5, -20, false, "ttt");

    println!("{:?}",tup.2);

    test_fn_one();

    test_fn_two(x, "0");

    let z = {
        let x = 3;
        x + 1
    };

    println!("{:?}",z);

    easy_if(20);

    let_if();

    easy_loop();

    easy_while();

    easy_for();
}

fn test_fn_one() {
    println!("wordl hello")
}

fn test_fn_two(x: i32, y: &str) {
    println!("{:?}", x.to_string() + y)
}

fn easy_if(x : u8) {
    let number : u8 = x;

    if number < 25 {
        println!("20 小于 25")
    } else {
        println!("20 大于 25")
    }
}

fn let_if() {
    let condition :bool = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("{:?}", number);
}

fn easy_loop() {
    let mut var_one = 1;

    let choose = loop {
        var_one += 1;

        if var_one == 10 {
            break var_one * 3;
        }
    };

    println!("{}", choose);
}

fn easy_while () {
    let numbers :[i32; 4] = [20, 25, 30, 70];

    let mut i = 0;

    while i < 4 {
        println!("{:?}", numbers[i]);

        i = i + 1;
    }

}

fn easy_for() {
    for num in (1..4).rev() {
        println!("for : {}", num);
    }
    println!(":)")
}