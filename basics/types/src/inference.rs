///
/// 推理
/// rust的类型推理是非常强大，它所做的不只是在初始化期间查看值表达式类型，
/// 它还介绍了变量如何在以后用于推理其类型
///


#[test]
fn main(){
    // 编译器知道var_one 是u8类型
    let var_one = 5u8;

    // 创建空向量（可增长的数组）
    let mut var_array = Vec::new();

    // 此时编译器不知道var_array的确切类型，只知道它是个可以修改的向量
    // 向向量添加元素
    var_array.push(var_one);
    var_array.push(20);

    // 当添加元素后编译器知道这个数组是u8类型的
    println!("var_array 数组 : {:?}", var_array);
}