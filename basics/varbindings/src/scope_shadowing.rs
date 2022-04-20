///
/// 变量的范围和阴影
/// 变量绑定是有作用域，并且受约束只能存在block，块是用{}括起来的集合
///


#[test]
fn main(){
    // 绑定在main函数中的变量
    let var_one = 2i32;

    {
        let var_two = 4i32;
        println!("var_two : {}", var_two);
    }
    // 会直接报错因为，var_two 的作用域限制在
    // println!("var_two : {}", var_two);
    println!("var_one: {}", var_one);
}

///
/// 此外，可以允许变量阴影
///

#[test]
fn main_two(){
    let var_three = 5.6f32;

    {
        println!("var_three: {}", var_three);

        let var_three = "test";
        println!("var_three: {}", var_three);
    }
    println!("var_three: {}", var_three);

    let var_three  = 3i32;
    println!("var_three: {:?}", var_three);
}
