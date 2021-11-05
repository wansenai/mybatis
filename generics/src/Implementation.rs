///
/// 与函数functions类似
///

struct S;
struct GenricVal<T>(T);

impl GenricVal<f32> {

}

impl GenricVal<S> {

}

// <T> 必须在类型面前才能保持泛型
impl <T> GenricVal<T>{

}

struct Val {
    val : f64,
}

struct GenVal<T>{
    gen_val : T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl <T> GenVal<T> {
    fn value(&self) -> &T{
        &self.gen_val
    }
}

#[test]
fn main(){
    let x = Val {val : 3.25};
    let y = GenVal {gen_val : 18i32};
    println!("x : {}", x.value());
    println!("y : {}", y.value());
}