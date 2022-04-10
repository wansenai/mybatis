#[test]
fn test_iter() {
    let vec_one = vec![1, 2, 3];

    let iters = vec_one.iter();

    for value in iters {
        println!("{}", value);
    }
}

#[test]
fn test_inte() {
    let v = vec![1, 2, 3, 4];

    let v_iter = v.iter();

    let total: i32 = v_iter.sum();

    assert_eq!(10, total);
}