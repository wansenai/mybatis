///
/// 与Form和Into相似，TryForm和TryInto是用于在类型之间进行转换的特征
/// 与Form和Into不同的是 TryFrom/TryInto 用于容易出错的转换 因此返回Result S
///

use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct Number(i32);

impl TryFrom<i32> for Number{
    type Error = ();
    fn try_from(value : i32) -> Result<Self, Self::Error>{
        if value % 2 == 0 {
            Ok(Number(value))
        } else {
            Err(())
        }
    }
}

#[test]
fn main(){

    use std::convert::TryInto;

    // TypeFrom
    assert_eq!(Number::try_from(8), Ok(Number(8)));
    assert_eq!(Number::try_from(5), Err(()));

    // TypeInto
    let result : Result<Number, ()> = 8i32.try_into();
    assert_eq!(result, Ok(Number(8)));
    println!("TypeInto result : {:?}",result);
    let result_two : Result<Number, ()> = 5i32.try_into();
    assert_eq!(result_two, Err(()));
    println!("TypeInto result_two :{:?}",result_two);
}