///
/// 迭代器搜索 IteratorSearch
/// Iteraator::find 是一个迭代器进行搜索的函数，如过搜索找不到值返回None
///

/*pub trait Iterator {
    type Item;

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        P : FnMut(&Self::Item) -> bool {}
}*/

#[test]
fn main(){
    let var_one = vec![1, 7, 9];
    let var_two = vec![12, 13, 15];

    let mut iter = var_one.iter();
    let mut into_iter = var_two.into_iter();

    println!("iter : {:?}", iter.find(|&&x| x == 7));
    println!("into_iter : {:?}", into_iter.find(|& x| x == 16));

    let array1 = [1, 5, 10];
    let array2 = [36, 20, 15];

    println!("array1 : {:?}", array1.iter().find(|&&x| x == 5));
    println!("array2 : {:?}", array2.iter().find(|&& y | y == 26));
}


// Iterator::position 可以找到索引

#[test]
fn main_test(){
    let var_one = vec![1, 9, 3, 3, 13, 2];

    let index = var_one.iter().position(|x| x % 2 == 0);
    assert_eq!(index, Some(5));

    let index2 = var_one.iter().position(|x | x < &0);
    assert_eq!(index2, None);
}


