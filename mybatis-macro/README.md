# Mybatis Macro

This module contains some macros and attributes built into mybatis, 
The source code of all macros is in this module.

## Getting Started

```rust
mybatis-macro = "2.0.1"
```

## Macro description

### 1. Macro drive MybatisPlus

This macro is mainly used to automatically map the conditional packaging of some built-in operation tables. 
You can realize the basic operations of querying, deleting, adding and modifying data tables without spelling SQL statements

```rust
#[derive(MybatisPlus)]
pub struct Book {
  pub id: Option<String>,
  pub name: Option<String>,
  pub types: Option<String>
}
```

### 2. Macro attribute mybatis_plus

The function of this macro is the same as that of MybatisPlus macro, except that this macro is used for attribute.

```rust
#[mybatis_plus]
pub struct Book {
  pub id: Option<String>,
  pub name: Option<String>,
  pub types: Option<String>
}

#[mybatis_plus(table_name:"book")]
pub struct Book {
  pub id: Option<String>,
  pub name: Option<String>,
  pub types: Option<String>
}
```

### 3. Macro attribute mybatis_sql
This macro is mainly used to automatically create SQL statements for you. 
You can write SQL statements in the macro corresponding to a method implementation.

```rust
#[mybatis_sql("select * from book where id = ?")]
async fn select(mybatis:&Mybatis, id: &str) -> Book {}
```

### 4. Macro attribute py_sql
This macro is mainly used to process pysql, and its function is similar to that of mybatis_sql macro

```rust
#[py_sql("select * from book where name = "《test》"")]
async fn py_select(name: &str) -> Book {}
```

### 5. Macro attribute mybatis_html
This macro is mainly used to parse the SQL syntax structure in HTML. 
The bottom layer is to parse the HTML file in the specified directory through HTML parse

```rust
#[mybatis_html("example/example.html")]
async fn html_parse_mybatis(mybatis: &Mybatis, name: &str) -> Book {}
```
