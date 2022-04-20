mod house;

pub use crate::house::hosting;

#[test]
pub fn eat() {
    hosting::add_waitlist();
}
