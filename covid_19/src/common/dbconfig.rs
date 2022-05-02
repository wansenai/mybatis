use mysql::*;

const DB_URL: &'static str = "mysql://root:passw0rd@localhost:3306/COVID-19";

pub fn get_conn() -> Result<PooledConn> {

    let opts = Opts::from_url(DB_URL).unwrap();
    let pool = Pool::new(opts).unwrap();
    let conn1 = pool.try_get_conn(357).unwrap();

    Ok(conn1)
}

///
/// test mysql connection
/// 
/// # notes
/// 
/// Since local MySQL needs to be linked here, and cargo test will cause errors when publishing GitHub CI action, 
/// comment the code first. 
/// This test method is used for local debugging
/// 
#[cfg(test)]
mod tests {

    // use super::*;

    // #[test]
    // fn test_conn() {
    //     get_conn();
    // }
}