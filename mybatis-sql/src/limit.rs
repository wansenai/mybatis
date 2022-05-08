use mybatis_drive::db::DriverType;
use crate::template::TEMPLATE;
use mybatis_drive::Error;
use mybatis_drive::Result;

pub trait PageLimit {
    /// return  sql
    fn page_limit_sql(&self, offset: u64, size: u64) -> Result<String>;
}

impl PageLimit for DriverType {
    fn page_limit_sql(&self, offset: u64, size: u64) -> Result<String> {
        return match self {
            DriverType::Mysql => Ok(format!(
                " {} {},{}",
                TEMPLATE.limit.value,
                offset,
                size
            )),
            DriverType::Postgres => Ok(format!(
                " {} {} {} {}",
                TEMPLATE.limit.value,
                size,
                TEMPLATE.offset.value,
                offset
            )),
            DriverType::Sqlite => Ok(format!(
                " {} {} {} {}",
                TEMPLATE.limit.value,
                size,
                TEMPLATE.offset.value,
                offset
            )),
            DriverType::Mssql => {
                //sqlserver
                Ok(format!(
                    " {} {} {} {} {}",
                    TEMPLATE.offset.value,
                    offset,
                    TEMPLATE.rows_fetch_next.value,
                    size,
                    TEMPLATE.rows_only.value
                ))
            }
            DriverType::None => Err(Error::from(format!(
                "[mybatis] not support now for DriverType:{:?}",
                DriverType::None
            ))),
        };
    }
}

#[test]
pub fn test_create_limit() {
    let mysql_limit = DriverType::Mysql.page_limit_sql(1, 20).unwrap();
    println!("{}", mysql_limit);
    let pg_limit = DriverType::Postgres.page_limit_sql(1, 20).unwrap();
    println!("{}", pg_limit);
    let sqlite_limit = DriverType::Sqlite.page_limit_sql(1, 20).unwrap();
    println!("{}", sqlite_limit);
}
