// 元组
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[test]
fn main (){

    fn reverse(pair : (i32, bool)) -> (bool, i32){
        let (integer, boolean) = pair;
    
        (boolean, integer)
    }

    // 定义一个多类型的元组
    let long_tuples = (1u8, 2u16 , 3u32, 4u64,
                                        -1i8, -2i16, -3i32, -4i64,
                                        0.1f32, 0.2f64,
                                        'a', true);

    println!("获取元组中第一个元素的值:{:?}", long_tuples.0);
    println!("获取元组中第二个元素的值:{}", long_tuples.1);

    // 元组可以是某个元组的元素
    let tuple_of_tuples = ((1u8, -3i8, 2u32), (4.5f32, 22u32), -20i8);
    println!("元组元素:{:?}", tuple_of_tuples);

    // 但是过长的元组不能打印会报错
    //let long_max_tuples = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    //println!("过长的元组{:?}", long_max_tuples);

    let pair = (1, true);
    println!("pair:{:?}", pair);
    println!("调用pair方法:{:?}",reverse(pair));

    // 要创建一个元素元组 需要用逗号区分
    // 用括号括起来
    println!("创建的元素元组:{:?}", (5f32,));
    println!("创建的元素元组:{:?}", (-5i64));

    // 元组可以被分解绑定给元素
    let test_tuples = (1, -50, "test", false);
    let (a, b, c, d) = test_tuples;
    println!("分解后的元组:{:?} , {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 2.2, 3.5, 6.6);
    println!("{:?}", matrix);
}

#[derive(Debug)]
struct Structure(f32, f32, f32, f32);

#[test]
fn learn(){
    use std::fmt;

    let matrx = Structure(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrx);

    impl fmt::Display for Structure{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "({}, {})", self.0, self.1)
        }
    }
}