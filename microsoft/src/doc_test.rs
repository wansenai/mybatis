///
/// 这个文件主要用于 练习文档测试，测试的时候
/// 使用 assert宏来验证 实际输出是否相同
/// 跟单元测试很相似 只不过这个类是在单元测试的基础上增加了doc文档
/// 正如你看到的那样 :)
///


///
/// 这个函数是用于验证 普通除法
///
/// # Example #1: 10 / 2 = 5
///
/// ```
/// let var = div(...);
/// assert_eq!(var, 2);
/// ```
///
/// # Example #2: 6 / 2 = 3
///
/// ```
/// TODO: 写入这个文档测试
/// ```
///
#[allow(dead_code)]
pub fn div(x: i32, y: i32) -> i32{
    if y == 0{
        panic!("入参参数不能为0")
    }

    x / y
}


///
/// 这个函数用于普通减法
///
/// # Example #1: 6 - 3 = 3
///
/// ```
/// let var = sub(...);
/// assert_eq!(var, 3)
/// ```
///
/// ```
/// TODO: 文档测试普通减法
/// ```
///
#[allow(dead_code)]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[test]
fn test_sub(){
    let var_a = sub(6,9);
    assert_eq!(var_a, -3);
}
