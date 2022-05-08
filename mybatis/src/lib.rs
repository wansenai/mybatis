#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate mybatis_macros;
#[macro_use]
pub extern crate mybatis_sql;

pub use mybatis_sql::{expr, push_index, rb_html, rb_py, sql_index};
pub use mybatis_sql::ops::*;
pub use mybatis_drive::*;

pub use mybatis_macros::{crud_table, CRUDTable, html_sql, py_sql, sql};

pub use mybatis_drive::{convert::StmtConvert, db::DriverType, error::Error, error::Result};

pub use crate::mybatis::AsSqlTag;

pub mod crud;
pub mod mybatis;
pub mod wrapper;
pub mod executor;
pub mod intercept;
pub mod log;
pub mod logic_delete;
pub mod object_id;
pub mod page;
pub mod snowflake;