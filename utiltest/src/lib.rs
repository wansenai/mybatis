use std::fs::File;
use std::io::ErrorKind;

#[test]
fn main() {
    read_file();
}

#[allow(dead_code)]
fn read_file() {
    let f = File::open("hello.txt");

    let _result = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("{:?}",e)
            },
            other_error => panic!("{:?}", other_error),
        }
    };
}

#[test]
fn test_copy_trait() {
    let numbers = vec![75, 57, 99, 105];

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
        let mut largest = &list[0];
    
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }

    let result = largest(&numbers);

    println!("{}", result);
}

#[test]
fn life_time_test(){

    fn longset<'a>(x: &'a str, y: &'a str) -> & 'a str{
        if x.len() > y.len() {
            x
        } else {
            y 
        }
    }

    let result;
    let s1 = String::from("asdasda");

    {
        let s2 = String::from("asdasdsadasdsadccc");
        result = longset(s1.as_str(), s2.as_str());
        println!("{}", result);
    }
}

pub fn sum_two(v: i32) -> i32 {
    v + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_ne() {
        let v2: i32 = 8;

        assert_ne!(11, sum_two(v2));
    }

    #[test]
    fn test_assert_eq() {
        let v3: i32 = 4;

        assert_eq!(6, sum_two(v3))
    }
}