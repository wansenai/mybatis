use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use rbson::Bson;
use rbson::spec::BinarySubtype;
use futures::Future;
use mybatis_core::db::DBExecResult;
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use mybatis_core::db::{DBPool, DBPoolConn, DBQuery, DBTx};
use mybatis_core::Error;
use crate::plus::{Mapping, MappingMut};
use crate::DriverType;
use crate::page::{IPageRequest, Page};
use crate::mybatis::Mybatis;
use mybatis_util::string_util;
use futures::executor::block_on;
use mybatis_core::{DateTimeNative, Format};
use crate::snowflake::new_snowflake_id;


/// the mybatis Containers for transactions, connections, and ontologies
/// for example:
///
/// &mut tx.as_executor()
/// &mut conn.as_executor()
/// &mut guard.as_executor()
/// (&rb).into()
/// (&mut tx).into()
/// (&mut conn).into()
/// (&mut guard).into()
#[derive(Debug)]
pub enum MyBatisExecutor<'r, 'inner> where 'inner: 'r {
    RB(&'r Mybatis),
    Conn(&'r mut MyBatisConnExecutor<'inner>),
    TX(&'r mut MyBatisTxExecutor<'inner>),
    TxGuard(&'r mut MyBatisTxExecutorGuard<'inner>),
}


impl MyBatisExecutor<'_, '_> {
    pub async fn fetch_page<T>(&mut self, sql: &str, args: Vec<Bson>, page_request: &dyn IPageRequest) -> crate::Result<Page<T>>
        where
            T: DeserializeOwned + Serialize + Send + Sync {
        match self {
            MyBatisExecutor::RB(rb) => {
                return rb.fetch_page(sql, args, page_request).await;
            }
            MyBatisExecutor::Conn(rb) => {
                return rb.fetch_page(sql, args, page_request).await;
            }
            MyBatisExecutor::TX(rb) => {
                return rb.fetch_page(sql, args, page_request).await;
            }
            MyBatisExecutor::TxGuard(rb) => {
                return rb.fetch_page(sql, args, page_request).await;
            }
        }
    }

    pub async fn exec(&mut self, sql: &str, args: Vec<Bson>) -> Result<DBExecResult, Error> {
        match self {
            MyBatisExecutor::RB(rb) => {
                return rb.exec(sql, args).await;
            }
            MyBatisExecutor::Conn(rb) => {
                return rb.exec(sql, args).await;
            }
            MyBatisExecutor::TX(rb) => {
                return rb.exec(sql, args).await;
            }
            MyBatisExecutor::TxGuard(rb) => {
                return rb.exec(sql, args).await;
            }
        }
    }

    pub async fn fetch<T>(&mut self, sql: &str, args: Vec<Bson>) -> Result<T, Error> where T: DeserializeOwned {
        match self {
            MyBatisExecutor::RB(rb) => {
                return rb.fetch(sql, args).await;
            }
            MyBatisExecutor::Conn(rb) => {
                return rb.fetch(sql, args).await;
            }
            MyBatisExecutor::TX(rb) => {
                return rb.fetch(sql, args).await;
            }
            MyBatisExecutor::TxGuard(rb) => {
                return rb.fetch(sql, args).await;
            }
        }
    }
}

impl<'r, 'inner> MybatisRef for MyBatisExecutor<'r, 'inner> {
    fn get_mybatis(&self) -> &Mybatis {
        match self {
            MyBatisExecutor::RB(rb) => {
                rb
            }
            MyBatisExecutor::Conn(rb) => {
                rb.get_mybatis()
            }
            MyBatisExecutor::TX(rb) => {
                rb.get_mybatis()
            }
            MyBatisExecutor::TxGuard(rb) => {
                rb.get_mybatis()
            }
        }
    }
}

impl<'r, 'inner> From<&'r Mybatis> for MyBatisExecutor<'r, 'inner> {
    fn from(arg: &'r Mybatis) -> Self {
        Self::RB(arg)
    }
}

impl<'r, 'inner> From<&'r mut MyBatisConnExecutor<'inner>> for MyBatisExecutor<'r, 'inner> {
    fn from(arg: &'r mut MyBatisConnExecutor<'inner>) -> Self {
        Self::Conn(arg)
    }
}

impl<'r, 'inner> From<&'r mut MyBatisTxExecutor<'inner>> for MyBatisExecutor<'r, 'inner> {
    fn from(arg: &'r mut MyBatisTxExecutor<'inner>) -> Self {
        Self::TX(arg)
    }
}

impl<'r, 'inner> From<&'r mut MyBatisTxExecutorGuard<'inner>> for MyBatisExecutor<'r, 'inner> {
    fn from(arg: &'r mut MyBatisTxExecutorGuard<'inner>) -> Self {
        Self::TxGuard(arg)
    }
}

#[async_trait]
pub trait MybatisRef {
    fn get_mybatis(&self) -> &Mybatis;

    fn driver_type(&self) -> crate::Result<DriverType> {
        self.get_mybatis().driver_type()
    }

    /// bind arg into DBQuery
    fn bind_arg<'arg>(
        &self,
        driver_type: &DriverType,
        sql: &'arg str,
        arg: Vec<Bson>,
    ) -> Result<DBQuery<'arg>, Error> {
        let mut q: DBQuery = driver_type.make_db_query(sql)?;
        for x in arg {
            (self.get_mybatis().encoder)(&mut q, x)?;
        }
        return Ok(q);
    }
}

#[async_trait]
pub trait ExecutorMut: MybatisRef {
    async fn exec(&mut self, sql: &str, args: Vec<rbson::Bson>) -> Result<DBExecResult, Error>;
    async fn fetch<T>(&mut self, sql: &str, args: Vec<rbson::Bson>) -> Result<T, Error> where T: DeserializeOwned;
}

#[async_trait]
pub trait Executor: ExecutorMut {}

impl MybatisRef for Mybatis {
    fn get_mybatis(&self) -> &Mybatis {
        &self
    }
}


#[derive(Debug)]
pub struct MyBatisConnExecutor<'a> {
    pub conn: DBPoolConn<'a>,
    pub rb: &'a Mybatis,
}

impl<'b> MyBatisConnExecutor<'b> {
    pub fn as_executor<'a>(&'a mut self) -> MyBatisExecutor<'a, 'b> {
        MyBatisExecutor::Conn(self)
    }
}

// bson vec to string
fn bson_arr_to_string(arg: Vec<Bson>) -> (Vec<Bson>, String) {
    let b = Bson::Array(arg);
    #[cfg(feature = "format_bson")]
    {
        let s = b.do_format();
        log::info!("[mybatis] [format_bson] => {}", s);
    }

    let s = b.to_string();
    return match b {
        Bson::Array(arr) => {
            (arr, s)
        }
        _ => {
            (vec![], s)
        }
    };
}

#[async_trait]
impl<'a> ExecutorMut for MyBatisConnExecutor<'_> {
    async fn exec(&mut self, sql: &str, mut args: Vec<rbson::Bson>) -> Result<DBExecResult, Error> {
        let rb_task_id = new_snowflake_id();
        let mut sql = sql.to_string();
        let is_prepared = args.len() > 0;
        for item in &self.get_mybatis().sql_intercepts {
            item.do_intercept(self.get_mybatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_mybatis().log_plugin.is_enable() {
            let (_args, args_string) = bson_arr_to_string(args);
            args = _args;
            self.get_mybatis().log_plugin.info(rb_task_id,
                                              &format!(
                                                  "Exec   ==> {}\n{}[mybatis]                      Args   ==> {}",
                                                  &sql,
                                                  string_util::LOG_SPACE,
                                                  args_string
                                              ),
            );
        }
        let result;
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type(), &sql, args)?;
            result = self.conn.exec_prepare(q).await;
        } else {
            result = self.conn.exec_sql(&sql).await;
        }
        if self.get_mybatis().log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.get_mybatis().log_plugin.info(rb_task_id,
                                                      &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.get_mybatis().log_plugin
                        .error(rb_task_id, &format!("ReturnErr  <== {}", e));
                }
            }
        }
        return result;
    }

    async fn fetch<T>(&mut self, sql: &str, mut args: Vec<rbson::Bson>) -> Result<T, Error> where T: DeserializeOwned {
        let rb_task_id = new_snowflake_id();
        let mut sql = sql.to_string();
        let is_prepared = args.len() > 0;
        for item in &self.get_mybatis().sql_intercepts {
            item.do_intercept(self.get_mybatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_mybatis().log_plugin.is_enable() {
            let (_args, args_string) = bson_arr_to_string(args);
            args = _args;
            self.get_mybatis().log_plugin.info(rb_task_id,
                                              &format!(
                                                  "Fetch  ==> {}\n{}[mybatis]                      Args   ==> {}",
                                                  &sql,
                                                  string_util::LOG_SPACE,
                                                  args_string
                                              ),
            );
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type(), &sql, args)?;
            let result = self.conn.fetch_parperd(q).await;
            if self.get_mybatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_mybatis().log_plugin
                            .info(rb_task_id, &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_mybatis().log_plugin
                            .error(rb_task_id, &format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        } else {
            let result = self.conn.fetch(&sql.to_owned()).await;
            if self.get_mybatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_mybatis().log_plugin
                            .info(rb_task_id, &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_mybatis().log_plugin
                            .error(rb_task_id, &format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        }
    }
}

impl MybatisRef for MyBatisConnExecutor<'_> {
    fn get_mybatis(&self) -> &Mybatis {
        self.rb
    }
}

impl<'a> MyBatisConnExecutor<'a> {
    pub async fn begin(self) -> crate::Result<MyBatisTxExecutor<'a>> {
        let tx = self.conn.begin().await?;
        return Ok(MyBatisTxExecutor {
            tx_id: new_snowflake_id(),
            conn: tx,
            rb: self.rb,
        });
    }
}

#[derive(Debug)]
pub struct MyBatisTxExecutor<'a> {
    pub tx_id: i64,
    pub conn: DBTx<'a>,
    pub rb: &'a Mybatis,
}

impl<'a, 'b> MyBatisTxExecutor<'b> {
    pub fn as_executor(&'a mut self) -> MyBatisExecutor<'a, 'b> {
        self.into()
    }
}


#[async_trait]
impl<'a> ExecutorMut for MyBatisTxExecutor<'_> {
    async fn exec(&mut self, sql: &str, mut args: Vec<rbson::Bson>) -> Result<DBExecResult, Error> {
        let mut sql = sql.to_string();
        let is_prepared = args.len() > 0;
        for item in &self.get_mybatis().sql_intercepts {
            item.do_intercept(self.get_mybatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_mybatis().log_plugin.is_enable() {
            let (_args, args_string) = bson_arr_to_string(args);
            args = _args;
            self.get_mybatis().log_plugin.info(self.tx_id,
                                              &format!(
                                                  "Exec   ==> {}\n{}[mybatis]                      Args   ==> {}",
                                                  &sql,
                                                  string_util::LOG_SPACE,
                                                  args_string
                                              ),
            );
        }
        let result;
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, args)?;
            result = self.conn.exec_prepare(q).await;
        } else {
            result = self.conn.exec_sql(&sql).await;
        }
        if self.get_mybatis().log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.get_mybatis().log_plugin.info(self.tx_id,
                                                      &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.get_mybatis().log_plugin
                        .error(self.tx_id, &format!("ReturnErr  <== {}", e));
                }
            }
        }
        return result;
    }

    async fn fetch<T>(&mut self, sql: &str, mut args: Vec<rbson::Bson>) -> Result<T, Error> where T: DeserializeOwned {
        let mut sql = sql.to_string();
        let is_prepared = args.len() > 0;
        for item in &self.get_mybatis().sql_intercepts {
            item.do_intercept(self.get_mybatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_mybatis().log_plugin.is_enable() {
            let (_args, args_string) = bson_arr_to_string(args);
            args = _args;
            self.get_mybatis().log_plugin.info(self.tx_id,
                                              &format!(
                                                  "Fetch  ==> {}\n{}[mybatis]                      Args   ==> {}",
                                                  &sql,
                                                  string_util::LOG_SPACE,
                                                  args_string
                                              ),
            );
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, args)?;
            let result = self.conn.fetch_parperd(q).await;
            if self.get_mybatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_mybatis().log_plugin
                            .info(self.tx_id, &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_mybatis().log_plugin
                            .error(self.tx_id, &format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        } else {
            let result = self.conn.fetch(&sql.to_owned()).await;
            if self.get_mybatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_mybatis().log_plugin
                            .info(self.tx_id, &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_mybatis().log_plugin
                            .error(self.tx_id, &format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        }
    }
}

impl MybatisRef for MyBatisTxExecutor<'_> {
    fn get_mybatis(&self) -> &Mybatis {
        self.rb
    }
}

impl<'a> MyBatisTxExecutor<'a> {
    pub async fn begin(&mut self) -> crate::Result<()> {
        return Ok(self.conn.begin().await?);
    }
    pub async fn commit(&mut self) -> crate::Result<()> {
        return Ok(self.conn.commit().await?);
    }
    pub async fn rollback(&mut self) -> crate::Result<()> {
        return Ok(self.conn.rollback().await?);
    }

    pub fn take_conn(self) -> Option<DBPoolConn<'a>> {
        return self.conn.take_conn();
    }
}

impl<'a> Deref for MyBatisTxExecutor<'a> {
    type Target = DBTx<'a>;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl<'a> DerefMut for MyBatisTxExecutor<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

pub struct MyBatisTxExecutorGuard<'a> {
    pub tx: Option<MyBatisTxExecutor<'a>>,
    pub callback: Box<dyn FnMut(MyBatisTxExecutor<'a>) + Send + 'a>,
}

impl Debug for MyBatisTxExecutorGuard<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyBatisTxExecutorGuard")
            .field("tx", &self.tx)
            .finish()
    }
}

impl<'a, 'b> MyBatisTxExecutorGuard<'b> {
    pub fn as_executor(&'a mut self) -> MyBatisExecutor<'a, 'b> {
        self.into()
    }

    pub async fn begin(&mut self) -> crate::Result<()> {
        let tx = self.tx.as_mut().ok_or_else(|| Error::from("[mybatis] tx is committed"))?;
        return Ok(tx.begin().await?);
    }

    pub async fn commit(&mut self) -> crate::Result<()> {
        let tx = self.tx.as_mut().ok_or_else(|| Error::from("[mybatis] tx is committed"))?;
        return Ok(tx.commit().await?);
    }

    pub async fn rollback(&mut self) -> crate::Result<()> {
        let tx = self.tx.as_mut().ok_or_else(|| Error::from("[mybatis] tx is committed"))?;
        return Ok(tx.rollback().await?);
    }

    pub fn take_conn(mut self) -> Option<DBPoolConn<'b>> {
        match self.tx.take() {
            None => {
                None
            }
            Some(s) => {
                s.take_conn()
            }
        }
    }
}

impl<'a> MyBatisTxExecutor<'a> {
    /// defer an func
    /// for example:
    ///     tx.defer(|tx| {});
    ///
    pub fn defer<Call>(self, callback: Call) -> MyBatisTxExecutorGuard<'a>
        where Call: 'a + FnMut(Self) + Send {
        MyBatisTxExecutorGuard {
            tx: Some(self),
            callback: Box::new(callback),
        }
    }

    /// defer and use future method
    /// for example:
    ///         tx.defer_async(|tx| async {
    ///             tx.rollback().await;
    ///         });
    ///
    pub fn defer_async<R, F>(self, mut callback: F) -> MyBatisTxExecutorGuard<'a>
        where R: 'a + Future<Output=()>,
              F: 'a + Send + FnMut(MyBatisTxExecutor<'a>) -> R {
        MyBatisTxExecutorGuard {
            tx: Some(self),
            callback: Box::new(move |arg| {
                block_on(callback(arg));
            }),
        }
    }

    pub async fn fetch_page<T>(
        &self,
        sql: &str,
        args: Vec<rbson::Bson>,
        page_request: &dyn IPageRequest,
    ) -> crate::Result<Page<T>>
        where
            T: DeserializeOwned + Serialize + Send + Sync,
    {
        self.get_mybatis().fetch_page(sql, args, page_request).await
    }
}

impl<'a> Deref for MyBatisTxExecutorGuard<'a> {
    type Target = MyBatisTxExecutor<'a>;

    fn deref(&self) -> &Self::Target {
        &self.tx.as_ref().unwrap()
    }
}

impl<'a> DerefMut for MyBatisTxExecutorGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tx.as_mut().unwrap()
    }
}


impl Drop for MyBatisTxExecutorGuard<'_> {
    fn drop(&mut self) {
        match self.tx.take() {
            None => {}
            Some(tx) => {
                (self.callback)(tx);
            }
        }
    }
}

impl Mybatis {
    pub async fn exec(&self, sql: &str, args: Vec<Bson>) -> Result<DBExecResult, Error> {
        let mut conn = self.acquire().await?;
        conn.exec(sql, args).await
    }

    pub async fn fetch<T>(&self, sql: &str, args: Vec<Bson>) -> Result<T, Error> where T: DeserializeOwned {
        let mut conn = self.acquire().await?;
        conn.fetch(sql, args).await
    }
}

#[async_trait]
impl ExecutorMut for Mybatis {
    async fn exec(&mut self, sql: &str, args: Vec<Bson>) -> Result<DBExecResult, Error> {
        let mut conn = self.acquire().await?;
        conn.exec(sql, args).await
    }

    async fn fetch<T>(&mut self, sql: &str, args: Vec<Bson>) -> Result<T, Error> where T: DeserializeOwned {
        let mut conn = self.acquire().await?;
        conn.fetch(sql, args).await
    }
}