#[macro_use]
extern crate mybatis;

mod pets;
mod books;
mod connection;
mod plugin_page_test;
mod snowflake_test;
mod string_util_test;
mod wrapper_test;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4+4, 8);
    }
}