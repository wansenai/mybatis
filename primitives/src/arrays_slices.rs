// 数组和切片

#[allow(dead_code)]
fn analyze_slice(slice: &[i32]){
    println!("切片的第一个元素:{}", slice[0]);
    println!("切片有{}个元素", slice.len());
}

#[test]
fn main(){

    use std::mem;

    let xs : [i32; 5] = [1, 2, 3, 4, 5];
    let ys : [i32; 500] = [0; 500];

    // 索引从0开始
    println!("数组中的第一个元素:{}", xs[0]);
    println!("数组中的第三个元素:{}", xs[2]);
    // len() 返回数组的长度
    println!("数组xs的长度是:{}", xs.len());
    // 数组是堆栈分配的
    println!("数组占用{}字节", mem::size_of_val(&xs));
    // 数组可以自动作为切片
    analyze_slice(&xs);

    // 用数组的一部分作为切片
    analyze_slice(&ys[1 .. 4]);

    // 数组越界编译错误 index out of bounds: the length is 500 but the index is 501
    // println!("{}",ys[501]);
}