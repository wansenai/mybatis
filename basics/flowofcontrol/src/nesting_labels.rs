///
/// 处理嵌套循环的时候，可以使用外循环break或者continue
/// 这种情况下loop 必须用一个便签表示，并且把标签传到break或者continue
///

#[test]
fn main(){

    let mut var = 5u32;

    'one : loop{
        var += 1;

        if var == 10 {

            loop{
                var += 2;
                break 'one;
            }
        }
    }
    println!("var的值: {}", var);
}