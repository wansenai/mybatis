///
/// Rust提供了高阶函数(HOF), 这些功能具有一个或多个功能
///
#[allow(dead_code)]
fn is_odd(var : u32) -> bool {
    var % 2 == 1
}

#[test]
fn main(){
    // 求1000以下所有奇数平方和
    let var_one = 1000;

    let mut var_two = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= var_one {
            // 当大于1000 跳出循环
            break;
        }else if is_odd(n_squared){
            var_two += n_squared;
        }
    }
    println!("1000以下所有奇数平方和 : {}", var_two);

    // 高阶函数 闭包过滤
    let sum_jishu : u32 =
        (0..).map(|var| var * var) // 平方和
            .take_while(|&n_squared| n_squared < var_one)// 条件表达式判断低于1000的数字
            .filter(|&n_squared| is_odd(n_squared))// 调用计算函数
            .fold(0, |count, n_squared | count + n_squared); // 求和
    println!("高阶函数计算1000以下所有奇数平方和  : {:?}", sum_jishu);
}