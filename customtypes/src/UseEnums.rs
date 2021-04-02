use crate::UseEnums::Status::Abled;
use crate::UseEnums::Work::Salary;

///
/// 可以使用use声明，这样就不需要手动作用域
///

enum Status {
    Disabled,
    Abled,
}

enum Work {
    Pace,
    Salary,
}

#[test]
fn main(){
    // use 可以显示的直接创建，这样可以不使用手动范围界定
  //  use crate::Status::{Disabled, Abled};
    // 可以在Work自动创建
 //   use crate::Work::*;

    // 相当于Status::Abled
    let status = Abled;
    // 相当于Work::Salary
    let work = Salary;

    match status {
        Disabled => println!("Disabled"),
        Abled            => println!("Abled"),
    }

    match work {
        Pace => println!("PuDong New Area, Shang Hai"),
        Salary      => println!("100")
    }
}
