//!
//! # crates
//! 
//! crates用于整合一些函数发布到crates.io 上
//! 
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Tests that
/// 
/// # Examples
/// 
/// ```
/// fn add_fun(x: i32) -> i32 {
///     x + 1
/// }
/// 
/// let x: i32 = 5;
/// let a = add_fun(x);
/// 
/// assert_eq!(6, a);
/// ```
/// 
/// # Errors
/// 
/// 如果传递的参数不是i32类型 会编译上出现错误，因为该函数只接受i32类型
/// 
/// # Panics
/// 
/// 如果内存不足，或者等其他情况会导致该函数在运行时起报错
/// 
pub fn add_fun(x: i32) -> i32 {
    x + 1
}

#[allow(dead_code)]
mod back_house {
    pub struct Breakfast {
      pub food: String,
      fruit: String,
    }
    
    impl Breakfast {
      pub fn today(food: String) -> Breakfast {
        Breakfast {
          food,
          fruit: String::from("apple"),
        }
      }
    }
  }

  pub fn eat_breakfast_today() {

    let food = String::from("bread");
    let mut meal = back_house::Breakfast::today(food);
    // 修改食物
    meal.food = String::from("dumplings");
    println!("今天吃: {}", meal.food);

    // 设置fruit的时候报错 因为 他的字段属性是私有的
    // meal.fruit = String::from("aa");
}
  
