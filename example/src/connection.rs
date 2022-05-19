use mybatis::crud::CRUD;
use mybatis::mybatis::Mybatis;


#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn test_database() {
    //     let rb = Mybatis::new();
    //     ///连接数据库,自动判断驱动类型"mysql://*","postgres://*","sqlite://*","mssql://*"加载驱动   
    //     rb.link("mysql://root:passw0rd@localhost:3306/COVID-19").await.unwrap();
    //     // use crate::core::db::DBPoolOptions;
    //     // let mut opt =DBPoolOptions::new();
    //     // opt.max_connections=100;
    //     // rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();

    //     //启用日志输出，你也可以使用其他日志框架，这个不限定的
    // }
}