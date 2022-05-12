#[macro_use]
pub extern crate mybatis_macro;
#[macro_use]
pub extern crate mybatis_sql;

pub use mybatis_sql::{expr, push_index, rb_html, rb_py, sql_index};
pub use mybatis_sql::ops::*;
pub use mybatis_core::*;

pub use mybatis_macro::CRUDTable;
pub use mybatis_macro::crud_table;

pub use mybatis_macro::{html_sql, py_sql, sql};

pub use mybatis_core::{convert::StmtConvert, db::DriverType, error::Error, error::Result};

pub use crate::mybatis::AsSqlTag;

#[macro_use]
pub mod mybatis;

pub mod crud;
pub mod wrapper;
pub mod executor;
pub mod intercept;
pub mod log;
pub mod logic_delete;
pub mod object_id;
pub mod page;
pub mod snowflake;