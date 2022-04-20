///
/// 函数的分散功能
///

// ! 表示空类型的函数
#[allow(dead_code)]
fn fool() -> ! {
    panic!("这个函数没有返回")
}

// 此类型函数无法实例化, 虽然函数里没有任何值，但他一样可以返回
#[allow(dead_code)]
fn some_fn(){
    ()
}


#[test]
fn main(){
    let _a : () = some_fn();

    fn sum_old(num : u32) -> u32{
        let mut temp = 0;

        for i in 0..num {
            let add : u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            temp += add;
        }
        temp
    }

    println!("调用函数sum_old : {:?}", sum_old(8));
}