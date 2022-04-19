mod text_operators;
mod tuples;
mod arrays_slices;

fn main() {
    let logical : bool = true;
    let price : f64 = 185.36;
    let number = 5i32;
    let default_float = 3.6;
    let default_integer = 723123;

    let mut number_one = 5;
    assert_eq!(number_one, 5);
    number_one = 15;
    assert_eq!(number_one, 15);

    let number_one = true;
    assert_eq!(number_one, true);

    assert_eq!(logical, true);
    assert_eq!(price, 185.36);
    assert_eq!(number, 5);
    assert_eq!(default_float, 3.6);
    assert_eq!(default_integer, 723123);
}
