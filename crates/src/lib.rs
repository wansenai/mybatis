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
/// let x: i32 = 5;
/// let a = crates::add_fun(x);
/// 
/// assert_eq!(7, a);
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
    x + 2
}