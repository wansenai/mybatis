use mybatis::mybatis::Mybatis;
use mybatis::plus::Mapping;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database() {
        // 连接数据库,自动判断驱动类型"mysql://*","postgres://*","sqlite://*","mssql://*"加载驱动
        // let rb = Mybatis::new();
        // rb.link("mysql://root:passw0rd@localhost:3306/COVID-19").await.unwrap();
        // use crate::core::db::DBPoolOptions;
        // let mut opt =DBPoolOptions::new();
        // opt.max_connections=100;
        // rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();
    }
}
