///
/// Rust 提供Match关键字提供模式匹配 像C语言中的switch一样，像java的switch
///

#[test]
fn main(){
    let number = 70;

    println!("数字为：{}", number);
    match number {
        1 => println!("包含1"),
        5 | 7 | 8 | 9  => println!("包含5789"),
        13..=22 => println!("包含20"),
        _ => println!("其他"),
    }

    let flag = true;
    let status = match flag {
        false => 0,
        true => 1,
    };
    println!("状态: {} -> {}", flag, status)
}

///
/// tuples元组类型的match测试
///

#[test]
fn tuples_match_test(){
    let age = (15,22,33);
    match age {
        (7 , _y ,_z) => println!("{:?}:包含为7岁的年龄", age),
        (15, ..) => println!("{:?}:从15岁以上的年龄匹配", age),
        _ => println!("{:?}:其他年龄", age)
    }
}

///
/// enum类型的match测试
///

#[allow(dead_code)]
enum Color{
    RED,
    BLUE,
    GREEN,

    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    CMY(u32, u32, u32),
}


#[allow(dead_code)]
enum HSV{
    HSV(u32, u32, u32),
}
#[test]
fn enums_match_test(){
    let color = Color::RGB(178, 50, 32);
    let hsv = HSV::HSV(5, 32, 66);

    match color {
        Color::RED => println!("红色"),
        Color::BLUE => println!("蓝色"),
        Color::GREEN => println!("绿色"),

        Color::RGB(r, g,b)  =>
            println!("RGB的颜色 R:{}, G:{}, B:{}", r, g, b ),
        Color::HSV(h,s,v)   =>
            println!("HSV H:{:}, S:{:}, V:{:}", h, s, v),
        Color::CMY(c,m,y)   =>
            println!("CMY C:{:?}, M:{:?}, Y:{:?}",c, m, y)
    }

    match hsv { HSV::HSV(h,s,v) =>
            println!("HSV H:{:}, S:{:}, V:{:}", h, s, v),
    }
}

///
/// pointer/ref 指针需要区分destructuring和dereferencing, 与C语言语法不同
///
/// Destructuring（取消引用） : *
/// Dereferencing（解构）: &，ref，ref mut
///

#[test]
fn porinter_ref_match_test(){

    let reference = &8;

    match reference {
        // 如果比较reference和&val匹配相当于 &i32 比较 &val，如果把&删除，i32类型应该分配给val
        &_val => println!("通过解构得到值：{}",reference)
    }

    match *reference {
        // 为了避免& 需要在匹配之前取消引用
        _val => println!("通过取消引用得到的值:{}", reference)
    }

    let _not_use_reference = 3;

    let ref _is_a_reference = 6;

    let value = 20;
    let mut _mut_value = 36;

    match value {
        ref _r => println!("value的值:{}", value)
    }

    match _mut_value {
        ref mut m => {
            *m += 4;
            println!("m的值: {}", m);
            println!("_mut_value的值: {}", _mut_value);
        },
    }
}

///
/// structs 结构 match 测试
///
#[test]
fn structs_match_test(){
    struct Foo{
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo{
        x : (6, 8), y : 20
    };

    match foo {
        Foo { x: (1, b), y }
            => println!("x的第一个数字是1, b = {},  y = {} ", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, 跟x没有关联", y),
    }
}