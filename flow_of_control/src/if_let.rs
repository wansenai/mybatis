///
/// if let 对于某些match匹配，可以使用if let来匹配，这样代码看着简洁
///

// if let 也可以用于 枚举值




#[test]
fn main_one(){
    let num = Some(7);

    match num {
        Some(i) => {
            println!("匹配数字{}",i)
        },
        _ => println!("没有匹配")
    }
}

#[test]
fn main_two(){
    let num = Some(20);
    let numbers : Option<i32> = None;
    let letters : Option<i32> = None;

    // if let结构是把num 分解为Some(i) 和 打印输出块
    if let Some(i) = num {
        println!("匹配到的数字 : {:?}", i);
    }
    // 也可以嵌套if else 处理
    if let Some(i) = numbers {
        println!("匹配到的数字 : {:?}", i);
    } else {
        println!("没有匹配任何数字");
    }

    let var_flag = false;
    if let Some(i) = letters {
        println!("匹配到的数字 : {:?}", i);
        // if let 结构无法破坏,它会执行if else
    } else if var_flag{
        println!("与数字不匹配");
    } else {
        println!("没有匹配任何数字");
    }
}

#[test]
fn main_three(){
    enum Status {
        LOGINSUCCESS,
        LOGINERROR,
        LOGINEXCEPTION(u32),
    }

    let var_one = Status::LOGINSUCCESS;
    let var_two = Status::LOGINERROR;
    let var_three = Status::LOGINEXCEPTION(500);

    if let Status :: LOGINSUCCESS = var_one {
        println!("var_one变量已经绑定到Login_Success");
    }

    if let Status :: LOGINERROR = var_two {
        println!("var_two变量已经绑定到Login_Error");
    }

    if let Status :: LOGINEXCEPTION(value) = var_three{
        println!("var_three 的值 {}", value);
    }

    // 可以通过@绑定值
    if let Status :: LOGINEXCEPTION(i @500) = var_three {
        println!("var_three 的值 {}", i);
    }
}

#[test]
fn main_four(){

    #[derive(Debug)]
    enum Number {
        ONE,
        TWO,
    }

    // if let 可以允许匹配非参数化的枚举变量，传统的枚举变量绑定将无法编译
    let var = Number::ONE;

    if let Number::TWO = var {
        println!("匹配非参数化的枚举变量");
    }

    let var_two = Number::TWO;
    println!("{:?}", var_two);
}