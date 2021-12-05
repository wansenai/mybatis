fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn main(){
    assert_eq!(add(5,6), 11);
    assert_eq!(add(7,9), 16);
}

pub fn is_eve(num : i32) -> bool{
    num % 2 == 0
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn is_true_fun() {
        assert!(is_eve(6));
    }

    // error
    fn is_false_fun(){
        assert!(is_eve(5));
    }
}