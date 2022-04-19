///
/// 类型匿名
/// 闭包简洁地捕获了来自合并范围的变量，需要泛型
///

// 必须是泛型
#[allow(dead_code)]
fn apply<F>(f : F) where
    F : FnOnce(){
    f();
}

// 对于不需要输入和返回什么都没有 用于print
#[test]
fn main(){
    let x = 7;
    let var_one = || println!("{}", x);
    apply(var_one);
}

