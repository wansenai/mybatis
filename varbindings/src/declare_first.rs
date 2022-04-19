///
/// 首先声明，这种方式很少用，因为它可能导致使用未初始化的变量
///


#[test]
fn main(){
    let a_binding;

    {
        let x = 5.6f32;

        a_binding = x * x;
    }
    println!("a_binding: {}", a_binding);

    let test_binding;
    // 打印报错 因为test_binding没有指定任何类型
    // println!("test_binding: {}", test_binding);

    test_binding = "biubiubiu~";
    println!("test_binding: {}", test_binding);
}

