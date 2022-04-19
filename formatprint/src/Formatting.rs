// 打印格式设定

use std::fmt::{
    self, Formatter, Display
};


struct City{
    name : &'static str,
    lat  : f32,
    lon  : f32,
}

impl Display for City{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'N' } else { 'S' };

        write!(f, "{}: {:.3} {} {:.3} {}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Color {
    red : u8,
    blue: u8,
    gree: u8,
}

#[test]
fn main () {

    for city in [
        City {name: "ShangHai", lat: 53.115166, lon: -151.51616},
        City {name: "SuZhou", lat: 50.58516, lon: 17.11231},
    ].iter(){
        println!("{}", *city);
    }

    for color in [
        Color {red: 8, blue: 7, gree: 12},
        Color {red: 12, blue: 99, gree: 254},
        Color {red: 36, blue: 1, gree: 6},
    ].iter(){
        println!("{:?}", *color);
    }
}