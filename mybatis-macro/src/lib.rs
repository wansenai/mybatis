#![allow(unused_assignments)]
extern crate proc_macro;

use syn::{parse_macro_input, AttributeArgs, DataStruct, ItemFn};

use crate::macros::mybatis_html_impl::impl_macro_mybatis_html;
use crate::macros::mybatis_plus_impl::{impl_mybatis_plus, impl_mybatis_plus_driver};
use crate::macros::mybatis_sql_impl::impl_macro_mybatis_sql;
use crate::macros::py_sql_impl::impl_macro_py_sql;
use crate::proc_macro::TokenStream;

mod element_from;
mod func;
mod html_loader;
mod parser;
mod py_sql;
mod string_util;
use std::collections::HashMap;

mod macros;
mod util;

#[proc_macro_derive(MybatisPlus)]
pub fn macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let stream = impl_mybatis_plus_driver(&ast, "", "", &HashMap::new());
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen impl MybatisPlus:\n {}", stream);
        println!("............gen impl MybatisPlus end............");
    }

    stream
}

/// auto create sql macro,this macro use RB.fetch_prepare and RB.exec_prepare
/// for example:
///     pub static RB:Lazy<Mybatis> = Lazy::new(||Mybatis::new());
///     #[sql(RB, "select * from biz_activity where id = ?")]
///     async fn select(name: &str) -> BizActivity {}
///
/// or:
///     #[sql("select * from biz_activity where id = ?")]
///     async fn select(mybatis:&Mybatis, name: &str) -> BizActivity {}
///
#[proc_macro_attribute]
pub fn mybatis_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_mybatis_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro sql:\n {}", stream);
        println!("............gen macro sql end............");
    }

    stream
}

/// py sql create macro,this macro use RB.py_fetch and RB.py_exec
///
///  pub static RB:Lazy<Mybatis> = Lazy::new(||Mybatis::new());
///  #[py_sql(RB,"select * from biz_activity where delete_flag = 0")]
///  async fn py_select_page(page_req: &PageRequest, name: &str) -> Page<BizActivity> { }
///  or:
///  #[py_sql("select * from biz_activity where delete_flag = 0")]
///  async fn py_select_page(mybatis: &mut MybatisExecutor<'_,'_>, page_req: &PageRequest, name: &str) -> Page<BizActivity> { }
///
///  or more example:
///  #[py_sql("
///     SELECT * FROM biz_activity
///     if  name != null:
///       AND delete_flag = #{del}
///       AND version = 1
///       if  age!=1:
///         AND version = 1
///       AND version = 1
///     AND a = 0
///       AND version = 1
///     and id in (
///     trim ',': for item in ids:
///       #{item},
///     )
///     and id in (
///     trim ',': for index,item in ids:
///       #{item},
///     )
///     trim 'AND':
///       AND delete_flag = #{del2}
///     choose:
///         when age==27:
///           AND age = 27
///         otherwise:
///           AND age = 0
///     WHERE id  = '2'")]
///   pub async fn py_select_mybatis(mybatis: &Mybatis, name: &str) -> Option<BizActivity> {}
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_py_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro py_sql :\n {}", stream);
        println!("............gen macro py_sql end............");
    }
    stream
}

/// html sql create macro,this macro use RB.py_fetch and RB.py_exec
/// for example:
///
/// pub static RB:Lazy<Mybatis> = Lazy::new(||Mybatis::new());
/// #[mybatis_html(RB,"example/example.html")]
/// pub async fn py_select_mybatis(name: &str) -> Option<BizActivity> {}
///
/// or:
///
/// #[mybatis_html("example/example.html")]
/// pub async fn py_select_mybatis(mybatis: &Mybatis, name: &str) -> Option<BizActivity> {}
///
#[proc_macro_attribute]
pub fn mybatis_html(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_mybatis_html(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro html_sql :\n {}", stream);
        println!("............gen macro html_sql end............");
    }
    stream
}

/// Mybatis Plus,You can define functionality using the following properties
/// #[mybatis_plus]
/// #[mybatis_plus(table_name:"biz_activity")]
/// #[mybatis_plus(table_name:"biz_activity" | table_columns:"id,name,version,delete_flag" | formats_pg:"id:{}::uuid,name:{}::string")]
/// pub struct BizActivity {
///   pub id: Option<String>,
///   pub name: Option<String>,
///   pub version: Option<i32>,
///   pub delete_flag: Option<i32>,
/// }
#[proc_macro_attribute]
pub fn mybatis_plus(args: TokenStream, input: TokenStream) -> TokenStream {
    let stream = impl_mybatis_plus(args, input);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen impl CRUDTable:\n {}", stream);
        println!("............gen impl CRUDTable end............");
    }

    return stream;
}

#[proc_macro_attribute]
pub fn expr(args: TokenStream, func: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = func::impl_fn(
        "",
        &target_fn.sig.ident.to_string(),
        &args.to_string(),
        true,
        true,
        &[],
    )
    .into();
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro rexpr:\n {}", stream);
        println!("............gen macro rexpr end............");
    }
    stream
}

#[proc_macro_attribute]
pub fn html(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn = syn::parse(func).unwrap();
    let stream = parser::impl_fn_html(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro xml:\n {}", stream);
        println!("............gen macro xml end............");
    }
    stream
}

/// support py_sql fn convert
#[proc_macro_attribute]
pub fn py(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn = syn::parse(func).unwrap();
    let stream = parser::impl_fn_py(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen pysql_fn:\n {}", stream);
        println!("............gen pysql_fn end............");
    }
    stream
}
