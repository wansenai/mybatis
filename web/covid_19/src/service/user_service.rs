use crate::domain::user;
use crate::common::dbconfig;
use mysql::params;
use mysql::prelude::Queryable;

use super::UserService;

type User = user::User;

impl UserService for User {
    fn insert_user(&self) -> bool {
        let mut conn = dbconfig::get_conn().unwrap();
        let result = conn.exec_drop("INSERT INTO user (id, username, password, name, sex, brithday, status) VALUES (?,?,?,?,?,?,?)", 
    (&self.id, &self.username, &self.password, &self.name, self.sex, &self.brithday, self.status,));
        
        if result.is_ok() {
            return true
        }
          false
    }

    fn user_login(&self, username: &str, password: &str) -> bool {
        let mut conn = dbconfig::get_conn().unwrap();
        let stmt = conn
            .prep("select username from user where username = :username and password = :password").unwrap();

        let result: Option<String> = conn.exec_first(&stmt, params! { "username" => username, "password" => password}).unwrap();
            
        match result {
            Some(_result) => true,
            None => false,
        }
    }
}