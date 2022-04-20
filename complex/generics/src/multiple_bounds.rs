///
/// Multiple bounds
/// 多重限制，可以使用符号 + 来来应用多哥多重限制， 跟正常用法一样，不同多限制用,分割
///

use std::fmt::{Debug, Display};

#[allow(dead_code)]
pub fn compare_prints<T: Debug + Display>(t: &T){
    println!("Debug : '{:?}'", t);
    println!("Display : '{}'", t);
}

#[allow(dead_code)]
fn compare_types<T : Debug, U: Debug>(t: &T, u: &U) {
    println!("t: '{:?}'", t);
    println!("u: '{:?}'", u);
}

#[test]
fn main(){
    let var_one = "words";
    let array = [1, 4, 7];
    let vec = vec![2, 8, 10];

    compare_prints(&var_one);

    compare_types(&array, &vec);
}