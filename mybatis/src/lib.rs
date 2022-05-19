#[macro_use]
pub extern crate mybatis_macro;
#[macro_use]
pub extern crate mybatis_sql;

// mapping macros
pub use mybatis_core as core;

pub use mybatis_sql::{expr, push_index, html, py, sql_index};
pub use mybatis_sql::ops::*;
pub use mybatis_core::*;

pub use mybatis_macro::MybatisPlus;
pub use mybatis_macro::mybatis_plus;

pub use mybatis_macro::{mybatis_html, py_sql, mybatis_sql};

pub use mybatis_core::{convert::StmtConvert, db::DriverType, error::Error, error::Result};

pub use crate::mybatis::AsSqlTag;

#[macro_use]
pub mod mybatis;

pub mod plus;
pub mod wrapper;
pub mod executor;
pub mod intercept;
pub mod log;
pub mod logic_delete;
pub mod object_id;
pub mod page;
pub mod snowflake;