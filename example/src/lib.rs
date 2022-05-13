#[macro_use]
extern crate mybatis;

mod pets;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4+4, 8);
    }
}