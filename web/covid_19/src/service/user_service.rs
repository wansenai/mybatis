use crate::domain::user;
use crate::common::dbconfig;

pub trait UserService {

    /// Create a new user in mysql
    fn insert_user(user: user::User) -> String;
    
}

impl UserService for user::User {
    fn insert_user(user: user::User) -> String {
        let coon = dbconfig::get_conn();
        println!("连接情况: {:?}", coon);
        String::from("test")
    }
}