///
/// 除了if let，while let也可以支持
///

#[test]
fn test_main(){
    let mut var = Some(0);

    loop {
        match var {
            Some (i) => {
                if i >= 10 {
                    println!("大于10 退出");
                    var = None;
                } else {
                    println!("获取i的值 : {:?}", i);
                    var = Some(i + 1);
                }
            },
            _ => {break;}
        }
    }
}

#[test]
fn test_main_two(){
    let mut var = Some(0);

    while let Some(i) = var {
        if i >= 10 {
            var = None;
        } else {
            println!("获取i的value : {:?}", i);
            var = Some(i + 2);
        }
    }
}