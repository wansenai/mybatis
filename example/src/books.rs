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
