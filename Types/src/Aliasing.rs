///
/// type 可以用于现有类型赋予新的名称，但是新的名称需要遵守驼峰命名法，否则编译器将发出警告
/// 唯一的例外的规则就是基本类型usize，f32，等
///

type Custom = u64;
type Price = f32;

// 该属性可以取消用type不命名规范警告
#[allow(non_camel_case_types)]
type var_u64 = u64;
type var_f32 = f32;

#[test]
fn main(){
    // var_one = Custom = var_u64 = u64类型
    let var_one : Custom = 5 as var_u64;

    // var_two = Price = var_f32 = f32类型
    let var_two : Price = 3.5 as var_f32;

    let var_three : Price = 6.6;

    println!("var_one :{}", var_one);

    println!("var_two : {} + var_three : {} = :{}", var_two, var_three, var_two + var_three);
}