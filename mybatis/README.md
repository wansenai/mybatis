# Summer MyBatis

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)


Summer MyBatis is an ORM framework based on rust language and mybatis framework. It is used to simplify development. Developers do not need to pay attention to complex SQL. They can carry out business development through dynamic SQL dialect. It can save your time. In addition, its speed is very fast.


## Features:

* Cross platform support (linux, macos, windows)
* No Runtimes，No Garbage Collection
* Powerful and flexible where conditional wrapper.
* Support multiple database drivers (mssql, mysql, postgres, sqlite).
* Support asynchronous operation with tokio.
* Structure automatic serialization and deserialization.


## Getting Started

* Step1. Add mybatis dependency

    ```rust
    mybatis = { version = "1.0.8"}
    // other dependency
    serde = { version = "1", features =  ["derive"] }
    rbson = "2.0"
    tokio = { version = "1.18.2", features = ["full"] }
    ```
* Step2. Create a structure corresponding to the database table and map the method
  
    ```rust
    use mybatis::crud::CRUD;
    use mybatis::mybatis_sql::string_util::to_snake_name;
    use mybatis::crud::CRUDTable;
    use mybatis::mybatis::Mybatis;
    use mybatis::snowflake::SNOWFLAKE;
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Pets {
      pub id: Option<String>,
      pub name: Option<String>,
      pub birthday: Option<mybatis::DateTimeNative>,
      pub delete_flag: Option<i32>,
    }
    
    impl CRUDTable for Pets {

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
    ```
* Next, you can do some database business operations

  ```rust
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
  /// Batch insert multiple objects
  ///
  #[tokio::test]
  async fn save_batch_pets() {
      let mybatis = Mybatis::new();

      mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

      let cat = Pets {
          id: Some(SNOWFLAKE.generate().to_string()),
          name: Some("Tom".to_string()),
          birthday: Some(mybatis::DateTimeNative::now()),
          delete_flag: Some(0),
      };

      let dog = Pets {
          id: Some(SNOWFLAKE.generate().to_string()),
          name: Some("Jerry".to_string()),
          birthday: Some(mybatis::DateTimeNative::now()),
          delete_flag: Some(0),
      };
    
      mybatis.save_batch(&vec![cat, dog],&[]).await;
  }
  
  ///
  /// Query multiple results and return dynamic array
  ///
  #[tokio::test]
  async fn query_all_pets() {
      let mybatis = Mybatis::new();

      mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

      let result: Vec<Pets> = mybatis.fetch_list().await.unwrap();
      println!("result: {:?}", result);
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
  
  ///
  /// Object modification through wrapper constructor
  ///
  #[tokio::test]
  async fn update_pet_by_name() {
      let mybatis = Mybatis::new();

      mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

      let update_wrapper = mybatis.new_wrapper().eq("name", "Tom");

      let james = Pets {
          id: None,
          name: Some("James".to_string()),
          birthday: None,
          delete_flag: None,
      };
      
      // Specifies which fields skip modifying the mapping
      let mut skip_columns = Vec::new();
      skip_columns.push(Skip::Column("id"));
      skip_columns.push(Skip::Column("birthday"));
      skip_columns.push(Skip::Column("delete_flag"));

      mybatis.update_by_wrapper(&james, update_wrapper, &skip_columns).await;
  }
  
  ///
  /// Physically delete a single object
  ///
  #[tokio::test]
  async fn delete_pet_by_name() {
      let mybatis = Mybatis::new();

      mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();
      mybatis.remove_by_column::<Pets,_>("name", "James").await;
  }
  
  
  ///
  /// Physical batch deletion of multiple objects
  ///
  #[tokio::test]
  async fn delete_betch_pet_by_name() {
      let mybatis = Mybatis::new();

      mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

      mybatis.remove_batch_by_column::<Pets,_>("name", &["Cindy", "Jerry"]).await;
  }
  ```
 
## Contribution

So far, we have only implemented some flexible conditional wrappers for summer-mybatis. We are developing the mapping of XML files and supporting Oracle database driver. If you have any interests and ideas, please submit to PR or contact me.

I have been working hard and are looking for various contributions. Look forward to your help!
