///
/// for 循环，for in 构造器可以遍历Iterator，比如a..b
/// 创建了从a 一直 到b 从1开始自增长循环
///

#[test]
fn main(){
    for n in 1..101 {
        if n % 15 == 0 {
            println!("n % 15 == 0 的值有: {}", n);
        }else if n % 10 == 0 {
            println!("n % 10 == 0 的值有: {}", n);
        }else if n % 5 == 0 {
            println!("n % 5 == 0 的值有: {}", n);
        }else{
            println!("其他: {}", n);
        }
    }

    // 使用 a..=b 两端包含在内的范围

    for m in 1..= 100 {
        if m / 15 == 0 {
            println!("n / 15 == 0 的值有: {}", m);
        }
    }
}


///
/// for迭代器 不仅可以循环自定义数值，也可以循环数组或者list的元素
/// 默认情况下for循环会将into_iter函数用于集合
/// 下面使用iter 和 into_iter 以及 iter_mut 分别举例
///

#[test]
fn two_main(){
    // 使用iter 可以在每次迭代中使用集合的元素，在循环完了之后集合还是保持元素不变，可以复用

    let names = vec!["ZhaoWei", "James", "Zow"];

    for name in names.iter() {
        match name {
            &"ZhaoWei" => println!("我的名称是ZhaoWei"),
            // 删除用 & 符号ZhaoWei的匹配
            _ => println!("剩下的名字有 : {}", name),
        }
    }
    println!("所有名称 {:?}", names);


    // 使用 into_iter 迭代后，不能复用原来的集合了，因为在每次迭代取用集合的元素

    let fruits = vec!["apple", "banana", "strawberry"];
    for fruit in fruits.into_iter() {
        match fruit {
            "apple" => println!("苹果"),
            _ => println!("有哪些水果 {}", fruit),
        }
    }
    // 这里报错
    // println!("水果:{:?}", fruits)

    // 使用 iter_mut

    let mut age_list = vec![15, 25, 55];
    for age in age_list.iter_mut(){
        match age {
            &mut 15 => println!("年龄15"),
            _ => println!("剩下年龄的数据为: {}", age),
        }
    }
    print!("ageList : {:?}", age_list);
}