mod mutability;
mod scope_shadowing;
mod declare_first;
mod var_freezing;

///
/// Rust 通过静态类型提供类型安全性，声明时，变量绑定可以用类型标注
/// 但是绝大多数 编译器可以通过上下文中推断变量的类型，这样可以省略一些注释
///

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn main(){
    let sex = 2u32;
    let hungry = true;
    let unit = ();

    // 复制sex的变量到新的sex2的变量
    let sex2 = sex;

    println!("sex : {}", sex);
    println!("hungry : {:?}", hungry);
    println!("unit : {:?}", unit);

    // 编译器警告未使用的变量
    let _unused_variable = 55u32;

    let noisy_unused_variable = 2u32;

    assert_eq!(sex2, 2u32);
    assert_eq!(noisy_unused_variable, 2u32);
}
