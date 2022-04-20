use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("==================猜数字游戏==================");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("请输入你的猜测");

        let mut guess_number = String::new();
    
        io::stdin().read_line(&mut guess_number).expect("读取数字出错");
    
        let guess_number: u32 = match guess_number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("你的猜测是：{}", guess_number);
    
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("太小啦"),
            Ordering::Greater => println!("太大啦"),
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                println!("==================游戏结束==================");
                break;
            }
        }
    }
}
