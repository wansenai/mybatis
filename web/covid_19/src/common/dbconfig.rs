use mysql::*;

const DB_URL: &'static str = "mysql://root:passw0rd@localhost:3306/COVID-19";

pub fn get_conn() -> Result<PooledConn> {
    let pool = Pool::new(DB_URL)?;
    let conn = pool.get_conn()?;

    Ok(conn)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_conn() {
        get_conn();
    }
}