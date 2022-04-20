///
/// 闭包 可以捕获封闭环境的变量和函数 例如 |val| val + x  捕获x变量的闭包
/// 在输入变量时使用|| 而不是 ()
/// 单个表达式可以选择 ({})
///

#[test]
fn main(){

    fn function_test(var : i32) -> i32 {
        var + 2
    }

    // 闭包是匿名的，注释与函数注释相同，就像包括方法体一样{}
    let closure_annotated = |i : i32| -> i32 { i * 2 };
    let closure_inferred = |i   | i * 4 ;

    let i = 1;
    println!("通用函数 : {}", function_test(i));
    println!("annotated 闭包 : {}", closure_annotated(i));
    println!("inferred 闭包 : {}", closure_inferred(i));

    let var =  || 1;
    println!("普通var : {}", var());
}


///
/// 捕捉 闭包本质上是灵活的，闭包也可以捕获某个变量
/// 引用变量: &T    可修改变量: &mut T    按值: T
///

#[test]
fn capturing_test(){
    use std::mem;

    let color = String::from("red");
    // 打印color 的 闭包，color将引用&color，和闭包的print
    let print = || println!("颜色: {}", color);
    print();

    // 对color不可变的引用
    let var_one = &color;
    println!("{}", var_one);

    // 把color绑定刀新的变量上
    let var_two = color;
    println!("{:?}", var_two);
    
    let mut count = 0;

    let mut inc = ||{
        count += 1;
        println!("count : {}", count)
    };
    // 调用inc闭包函数
    inc();

    let movable = Box::new(3);
    let consume = || {
        println!("Box :{:?}", movable);
        mem::drop(movable);
    };

    consume();
}

///
/// move 可以强制关闭从闭包中已获取和已捕获的变量所有权
///

#[test]
fn move_test(){
    let var_list = vec![1, 7, 8];

    let contains = move |needle| var_list.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // 打印数组长度会报错，只有移除move，将导致闭包，就可以引用var_list变量
    // println!("数组长度: {:?}", var_list.len());
}
