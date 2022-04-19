///
/// 迭代器::any  Iterator::any 是一个函数， 当传递迭代器的时候，当为true则返回该函数 否则false
///

/*pub trait Iterators {
    type Item;

    fn any<F>(&mut self, f : F) -> bool where
        F : FnMut(self::Item) -> bool {}
}*/

#[test]
fn main(){
    let var_one = vec![5, 9, 11];
    let var_two = vec![1, 3, 6];
    // 闭包检查某个元素在不在数组里，如果存在就返回true，否则返回false
    println!("var_one : {}", var_one.iter().any(|&x| x == 2));
    println!("var_one : {}", var_two.into_iter().any(|x| x == 3));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("array1 : {}", array1.iter().any(|&x| x == 3));
    println!("arrary2 : {}", array2.iter().any(|&x| x == 7));
}