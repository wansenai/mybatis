use mysql::*;

pub fn get_conn() -> Result<PooledConn> {
    let url = "mysql://root:passw0rd@localhost:3306/COVID-19";
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}
