use crate::domain::user;
use crate::common::dbconfig;
use mysql::prelude::Queryable;

pub trait UserService {
    /// Create a new user in mysql
    fn insert_user(&self) -> bool;
}

impl UserService for user::User {
    fn insert_user(&self) -> bool {
        let mut conn = dbconfig::get_conn().unwrap();
        let result = conn.exec_drop("INSERT INTO user (id, username, password, name, sex, brithday, status) VALUES (?,?,?,?,?,?,?)", 
    (&self.id, &self.username, &self.password, &self.name, self.sex, &self.brithday, self.status,));
        
        if result.is_ok() {
            return true
        }
          false
    }
}