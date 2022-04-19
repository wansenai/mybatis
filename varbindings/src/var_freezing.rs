///
/// 变量冻结 当数据不变的由相同变量名称绑定时，就会导致变量冻结。
/// 在不可变绑定超出范围之前 无法修改冻结的数据
///


#[test]
fn main(){
    let mut var_integer = 5i32;

    {
        let _var_integer = var_integer;

        // 这里报错 因为不能对不可变变量赋值2次
        // var_integer = 6;
    }
    var_integer = 7;
    println!("var_integer : {}", var_integer);
}