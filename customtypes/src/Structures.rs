///
/// 定义结构 Structures 支持三种类型定义结构
/// 1. 元组结构，基本上是元组
/// 2. C语言结构
/// 3. 无字段的单元结构对于泛型很有用
///

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name : String,
    age  : u8,
    sex  : String,
}

// 单独的结构
#[allow(dead_code)]
struct Unit;

// 元组结构
#[allow(dead_code)]
struct Pair(f32, f32);

// 2个字段的结构
#[allow(dead_code)]
struct Point {
    x : f32,
    y : f32,
}

// 结构可以作为另外一个字段的重用结构
#[allow(dead_code)]
struct Rectstruct {
    top_left     : Point,
    bootom_right : Point,
}

#[test]
fn main() {
    // test User
    let name = String::from("James");
    let age = 22;
    let sex = String::from("男");
    let james = User { name, age, sex };
    println!("{:?}", james);

    // test Point
    let point: Point = Point { x: 1.6, y: 7.3 };
    println!("坐标:({}, {})", point.x, point.y);

    // 使用struct语法创建
    let bootom_right = Point { x: 5.2, ..point };
    println!("第二个坐标:({}, {})", bootom_right.x, bootom_right.y);

    // 使用let绑定分解点
    let Point { x: top_edge, y: left_edge } = point;

    let _rectstruct = Rectstruct {
        top_left : Point {x : left_edge, y : top_edge},
        bootom_right : bootom_right,
    };

    // test Unit
    let _unit = Unit;
    let pair = Pair(5.3f32, 2.7f32);
    println!("pair的值 {:?} 和 {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair的值 {:?} 和 {:?}", integer, decimal);
}

#[allow(dead_code)]
struct Calculates{
    above : f32,
    below : f32,
    hight : f32,
}

///
/// 计算矩阵面积
/// 公式：(上边+底边)*高/2
///

#[test]
fn learn(){
    let init : Calculates = Calculates{above : 5.5, below: 5.8, hight : 6.7};

    let sum = (init.above + init.below) * init.hight / 2.0;

    println!("该矩形面积为:{:}", sum)
}