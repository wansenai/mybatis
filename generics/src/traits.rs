///
/// traits 也可以是泛型的
///

#[allow(dead_code)]
struct Empty;
#[allow(dead_code)]
struct Null;

// 附加单个参数T 不做任何操作
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// 此实现的方法有两个参数需要传递
impl <T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {

    }
}

#[test]
fn main(){
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}
