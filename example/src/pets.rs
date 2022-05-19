use mybatis::{mybatis_sql::string_util::to_snake_name};
use mybatis::mybatis::Mybatis;
use mybatis::plus::MybatisPlus;
use mybatis::snowflake::SNOWFLAKE;
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
    use mybatis::plus::{Skip, Mapping};

    use super::*;

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

    #[tokio::test]
    async fn query_all_pets() {
        let mybatis = Mybatis::new();

        mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

        let result: Vec<Pets> = mybatis.fetch_list().await.unwrap();
        println!("result: {:?}", result);
    }

    #[tokio::test]
    async fn query_pet_by_name() {
        let mybatis = Mybatis::new();

        mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

        let result: Option<Pets> = mybatis.fetch_by_column("name", &"Cindy").await.unwrap();
        println!("result: {:?}", result);
    }

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
        
        let mut skip_columns = Vec::new();
        skip_columns.push(Skip::Column("id"));
        skip_columns.push(Skip::Column("birthday"));
        skip_columns.push(Skip::Column("delete_flag"));

        mybatis.update_by_wrapper(&james, update_wrapper, &skip_columns).await;
    }

    #[tokio::test]
    async fn delete_pet_by_name() {
        let mybatis = Mybatis::new();

        mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();
        mybatis.remove_by_column::<Pets,_>("name", "James").await;
    }

    #[tokio::test]
    async fn delete_betch_pet_by_name() {
        let mybatis = Mybatis::new();

        mybatis.link("mysql://root:passw0rd@localhost:3306/test").await.unwrap();

        mybatis.remove_batch_by_column::<Pets,_>("name", &["Cindy", "Jerry"]).await;
    }
}