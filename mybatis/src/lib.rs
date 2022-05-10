#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![forbid(unsafe_code)]

#[macro_use]
pub extern crate mybatis_macro;
#[macro_use]
pub extern crate mybatis_sql;

pub use mybatis_sql::{expr, push_index, rb_html, rb_py, sql_index};
pub use mybatis_sql::ops::*;
pub use mybatis_core::*;

pub use mybatis_macro::{crud_table, CRUDTable, html_sql, py_sql, sql};

pub use mybatis_core::{convert::StmtConvert, db::DriverType, error::Error, error::Result};

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