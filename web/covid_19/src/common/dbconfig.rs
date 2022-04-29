use mysql::*;

const DB_URL: &'static str = "mysql://root:passw0rd@localhost:3306/COVID-19";

pub fn get_conn() -> Result<PooledConn> {

    let opts = Opts::from_url(DB_URL).unwrap();
    let pool = Pool::new(opts).unwrap();
    let conn1 = pool.try_get_conn(357).unwrap();

    Ok(conn1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_conn() {
        get_conn();
    }
}