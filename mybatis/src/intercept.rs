use mybatis_core::convert::StmtConvert;

use crate::mybatis::Mybatis;
use crate::plus::MybatisPlus;
use mybatis_core::db::DriverType;
use mybatis_core::Error;
use mybatis_sql::TEMPLATE;
use rbson::Bson;
use std::fmt::{Debug, Display};

/// sql intercept
pub trait SqlIntercept: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// do intercept sql/args
    /// is_prepared_sql: if is run in prepared_sql=ture
    fn do_intercept(
        &self,
        rb: &Mybatis,
        sql: &mut String,
        args: &mut Vec<rbson::Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct MyBatisLogFormatSqlIntercept {}

impl SqlIntercept for MyBatisLogFormatSqlIntercept {
    fn do_intercept(
        &self,
        rb: &Mybatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        let driver_type = rb.driver_type()?;
        match driver_type {
            DriverType::None => {}
            DriverType::Mysql | DriverType::Postgres | DriverType::Sqlite | DriverType::Mssql => {
                let mut formated = format!("[format_sql]{}", sql);
                for index in 0..args.len() {
                    let mut data = String::new();
                    driver_type.stmt_convert(index, &mut data);
                    formated =
                        formated.replacen(&data, &format!("{}", args.get(index).unwrap()), 1);
                }
                rb.log_plugin.info(0, &formated);
            }
        }
        return Ok(());
    }
}

/// Prevent full table updates and deletions
#[derive(Debug)]
pub struct BlockAttackDeleteInterceptor {}

impl SqlIntercept for BlockAttackDeleteInterceptor {
    fn do_intercept(
        &self,
        rb: &Mybatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        let sql = sql.trim();
        if sql.starts_with(TEMPLATE.delete_from.value)
            && !sql.contains(TEMPLATE.r#where.left_right_space)
        {
            return Err(Error::from(format!(
                "[mybatis][BlockAttackDeleteInterceptor] not allow attack sql:{}",
                sql
            )));
        }
        return Ok(());
    }
}

/// Prevent full table updates and deletions
#[derive(Debug)]
pub struct BlockAttackUpdateInterceptor {}

impl SqlIntercept for BlockAttackUpdateInterceptor {
    fn do_intercept(
        &self,
        rb: &Mybatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        let sql = sql.trim();
        if sql.starts_with(TEMPLATE.update.value)
            && !sql.contains(TEMPLATE.r#where.left_right_space)
        {
            return Err(Error::from(format!(
                "[mybatis][BlockAttackUpdateInterceptor] not allow attack sql:{}",
                sql
            )));
        }
        return Ok(());
    }
}
