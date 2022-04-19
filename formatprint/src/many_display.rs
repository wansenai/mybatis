// Display 有时候处理多个打印结果, 像数组那样就需要一个个写impl fmt::Display, 使用?onwrite! 可以一次打印所有元素

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let vac = &self.0;
        write!(f, "[")?;


        for (count, v) in vac.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            // 下标
            write!(f, "{}:", v - 1)?;
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

#[test]
fn main(){
    let v = List(vec![1, 2, 3]);
    print!("{}", v);
}