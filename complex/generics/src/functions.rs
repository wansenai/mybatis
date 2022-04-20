///
/// 相同的规则可以应用于函数：类型T在前面时变为泛型<T>
/// 使用泛函数有时需要明确指定类型参数，如果在返回类型是泛型的情况下调用函数，或者编译器没有足够的信息来推断必要的类型参数
/// 和显式指定类型参数的函数调用
///

struct A;

struct S(A);

struct SGen<T>(T);

#[allow(dead_code)]
// 定义一个函数 该函数接受类型为S的参数_s
fn reg_fn(_s : S) {}

#[allow(dead_code)]
// 定义一个函数 接受SGen函数，该接受类型被赋予了参数A，但因A没有，所以指定gen_spec_t函数 但是他不是泛型的
fn gen_spec_t(_s : SGen<A>) {}

#[allow(dead_code)]
// 定义接受函数SGen<i32> 因为i32不是泛型类型，所以此函数不是泛型
fn gen_spec_i32(_s : SGen<i32>) {}

#[allow(dead_code)]
// 定义一个函数，该函数接受SGen<T> ，因为该函数接受对象前面有<T>, 所以此函数是T的泛型函数
fn generic<T>(_s : SGen<T>) {}

#[test]
fn main(){
    reg_fn(S(A));

    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic::<char>(SGen('a'));
}
