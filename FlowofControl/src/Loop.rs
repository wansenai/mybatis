///
/// Rust 提供Loop表示循环，可以使用break退出循环，也可以使用continue跳过其余迭代并开始新的循环
///

#[test]
fn main(){

    let mut count = 0u32;

    loop {
        count += 1;

        if count == 5 {
            // 当等于5的时候跳过
            continue;
        }
        println!("{}", count);
        if count == 8 {
            break;
        }
    }
}