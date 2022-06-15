# Summer MyBatis

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fsummer-os%2Fsummer-mybatis.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fsummer-os%2Fsummer-mybatis?ref=badge_shield)


Summer MyBatis is an ORM framework based on rust language and mybatis framework. It is used to simplify development. Developers do not need to pay attention to complex SQL. They can carry out business development through dynamic SQL dialect. It can save your time. Use 
[sqlx-core crate](https://crates.io/crates/sqlx-core)、
[tokio crate](https://crates.io/crates/tokio)、
[rbatis crate](https://crates.io/crates/rbatis)、
[bson crate](https://crates.io/crates/bson) 
is used for secondary packaging development. 

In addition, its speed is very fast.


## Features:

* Cross platform support (linux, macos, windows)
* No Runtimes，No Garbage Collection
* Powerful and flexible where conditional wrapper.
* Support multiple database drivers (mssql, mysql, postgres, sqlite).
* Support asynchronous operation with tokio.
* Structure automatic serialization and deserialization.


## Getting Started

* Add mybatis dependency
    
    ```rust
    mybatis = { version = "2.0.3"}
    ```

* Example
    
    <details open=“open”>
    <summary> Use #[mybatis_plus] macro data table mapping </summary>
    
    ```rust
    use serde::{Serialize, Deserialize};

    #[mybatis_plus]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Books {
        pub id: Option<String>,
        pub name: Option<String>,
        pub types: Option<String>
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use mybatis::mybatis::Mybatis;
        use mybatis::snowflake::SNOWFLAKE;
        use mybatis::plus::Mapping;

        #[tokio::test]
        async fn save_books() {
            let mybatis = Mybatis::new();

            mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

            let id = SNOWFLAKE.generate();
            let cat = Books {
                id: Some(id.to_string()),
                name: Some("《Daughter of the sea》".to_string()),
                types: Some("Fairy Tales".to_string()),
            };
    
            mybatis.save(&cat,&[]).await;
        }
    }
    ```
    </details>

    <details>
    <summary>If you don't want to use macros, you can create a structure corresponding to the database table and map the method </summary>
        
    ```rust
    use mybatis::mybatis_sql::string_util::to_snake_name;
    use mybatis::plus::MybatisPlus;
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Pets {
      pub id: Option<String>,
      pub name: Option<String>,
      pub birthday: Option<mybatis::DateTimeNative>,
      pub delete_flag: Option<i32>,
    }
    
    impl MybatisPlus for Pets {

      fn table_name() -> String {
          let type_name = std::any::type_name::<Self>();
          let mut name = type_name.to_string();
          let names: Vec<&str> = name.split("::").collect();
          name = names.get(names.len() - 1).unwrap_or(&"").to_string();

          to_snake_name(&name)
      }

      fn table_columns() -> String {
          String::from("id,name,birthday,delete_flag")
      }
      
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use mybatis::mybatis::Mybatis;
        use mybatis::snowflake::SNOWFLAKE;
        use mybatis::plus::{Skip, Mapping};

        ///
        /// Save a single object 
        ///
        #[tokio::test]
        async fn save_pets() {
            let mybatis = Mybatis::new();

            mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

            let id = SNOWFLAKE.generate();
            let cat = Pets {
                id: Some(id.to_string()),
                name: Some("Cindy".to_string()),
                birthday: Some(mybatis::DateTimeNative::now()),
                delete_flag: Some(0),
            };
    
            mybatis.save(&cat,&[]).await;
        }

        ///
        /// Query a single object according to the specified field and return Option<Object>
        ///
        #[tokio::test]
        async fn query_pet_by_name() {
            let mybatis = Mybatis::new();

            mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

            let result: Option<Pets> = mybatis.fetch_by_column("name", &"Cindy").await.unwrap();
            println!("result: {:?}", result);
        }
    }
    ```
    </details>

## Community
If you have any use questions, you can ask questions in [mybatis discussions](https://github.com/summer-rust/summer-mybatis/discussions) or submit an issue.
        
## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fsummer-os%2Fsummer-mybatis.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fsummer-os%2Fsummer-mybatis?ref=badge_large)
 
## Contribution

So far, we have only implemented some flexible conditional wrappers for summer-mybatis. We are developing the mapping of XML files and supporting Oracle database driver. If you have any interests and ideas, please submit to PR or contact me.

We looking for various contributions. Look forward to your contributions!
