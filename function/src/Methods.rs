///
/// 方法是附加到对象里的函数，可以通过self关键字访问对象和其他方法的数据
/// 方法定义在 impl 块中
///

#[allow(dead_code)]
struct Point {
    x : f64,
    y : f64
}


#[allow(dead_code)]
impl Point{
    fn origin() -> Point{
        Point{x : 0.0, y : 0.0}
    }

    fn new(var_one: f64, var_two: f64) -> Point{
        Point{x: var_one, y: var_two}
    }
}

#[allow(dead_code)]
struct Rectangle {
    p1 : Point,
    p2 : Point
}

#[allow(dead_code)]
impl Rectangle{
    fn area(&self) -> f64{
        let Point{x: x1, y : y1} = self.p1;
        let Point{x: x2,y : y2} = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x : f64, y : f64){
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}


#[allow(dead_code)]
struct Pair(Box<i32>, Box<i32>);


#[allow(dead_code)]
impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("获取值 {} 和 {}",first, second);
    }
}

#[test]
fn main(){

    let test = Rectangle {
        p1 : Point::origin(),
        p2 : Point::new(1.0,2.6),
    };

    let mut square = Rectangle {
        p1 : Point::origin(),
        p2 : Point::new(1.7, 9.3),
    };
    // 没有mut修饰 所以test调用不了translate
    // test.translate(2.3,1.5);
    let var = test.area();
    println!("{:?}", var);

    let var_two = square.perimeter();
    println!("{}", var_two);

    square.translate(2.7, 10.3);
    let pair = Pair(Box::new(10), Box::new(20));
    pair.destroy();
}