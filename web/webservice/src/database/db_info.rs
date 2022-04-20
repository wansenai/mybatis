use mysql::*;
use mysql::prelude::*;

#[allow(dead_code)]
pub fn test_coon(){
    let url = "mysql://root:123456@localhost:3306/cloud_service_db";
    let pool = Pool::new(url);
    let conn = pool.unwrap().get_conn();
    println!("连接情况: {:?}", conn);
}

#[allow(dead_code)]
pub fn new_query_with_param() -> Result<String> {
    println!("aaaa");
    let url = "mysql://root:123456@localhost:3306/cloud_service_db";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let stmt = conn
        .prep("select user_name from sys_user where id = :id")?;
    let name: String = conn
        .exec_first(&stmt, params! { "id" => 1 })?
        .unwrap();
    println!("\n{}\n", name);
    Ok(name)
}
