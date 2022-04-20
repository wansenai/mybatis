#[derive(Debug)]
struct OtherError;

#[allow(dead_code)]
fn sum(numa: f64, numb: f64) -> Result<f64, OtherError> {
    if numb == 0.0 {
        Err(OtherError)
    } else {
        Ok(numa / numb)
    }
}

#[test]
fn main(){
    println!("{:?}", sum(9.0, 3.0));
}
