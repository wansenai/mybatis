mod from_into;
mod tryfrom_tryinto;
mod to_from_string;

///
/// conversion 转换，原始类型可以通过强制转换改变类型
/// Rust 通过使用trait 解决了自定义类型（struct和enum）之间的转换
/// 然而对于更常见的情况，有具体的办法，尤其是在转换String字符串的时候
///

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
