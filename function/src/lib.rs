mod methods;
mod closures;
mod inupt_parameters;
mod type_anonymity;
mod input_function;
mod output_function;
mod iterator_any;
mod iterator_search;
mod higher_order_function;
mod diverging_function;

///
/// 函数 使用fn 关键字定义一个函数, 如果函数有返回值，则必须在方法体后 执行 ->
/// 也可以提前在函数里 通过return 返回
///

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn give_number(){
    auto_number(100);
}

#[allow(dead_code)]
fn auto_number(n : u32){
    for n in 1..n + 1 {
        test_method(n);
    }
}

// 返回布尔值的函数
#[allow(dead_code)]
fn is_val(var_one : u32, var_two : u32) -> bool {
    if var_one == 0 {
        return false;
    }
   return var_one % var_two == 0;
}

#[allow(dead_code)]
fn test_method(n : u32) -> () {
    if is_val(n , 15){
        println!("数据1: {}", n);
    } else if is_val(n, 3){
        println!("数据2: {}", n);
    } else if is_val(n, 5){
        println!("数据3: {}", n);
    } else {
        println!("其他数据: {}", n);
    }
}
