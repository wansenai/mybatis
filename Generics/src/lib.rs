///
/// 泛型是将类型或者功能泛化为更广的案例，可以减少重复代码
/// 通用类型参数表示<T> 在Rust中 泛型还描述接受一个或多个泛型的任何内容<T>
///

// 例如定义名为泛型函数foo
fn foo<T>(args : T){

}

struct A;

struct Single(A);

struct SingleGen<T>(T);

#[cfg(test)]
mod tests {
    use crate::Single;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn main(){
    let _s = Single(A);

    let _char : SingleGen<char> = SingleGen('J');

    let _t = SingleGen(A);
    let _i32 = SingleGen(5);
    let _char = SingleGen('t');
}
