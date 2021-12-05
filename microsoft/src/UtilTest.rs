fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn main(){
    assert_eq!(add(5,6), 11);
    assert_eq!(add(7,9), 16);
}