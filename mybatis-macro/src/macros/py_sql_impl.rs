use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn::{AttributeArgs, FnArg, ItemFn};

use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, get_page_req_ident, is_fetch, is_mybatis_ref};

///py_sql macro
///support args for RB:&Mybatis,page:&PageRequest
///support return for Page<*>
pub(crate) fn impl_macro_py_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let mut mybatis_ident = "".to_token_stream();
    let mut mybatis_name = String::new();
    for x in &target_fn.sig.inputs {
        match x {
            FnArg::Receiver(_) => {}
            FnArg::Typed(t) => {
                let ty_stream = t.ty.to_token_stream().to_string();
                if is_mybatis_ref(&ty_stream) {
                    mybatis_ident = t.pat.to_token_stream();
                    mybatis_name = mybatis_ident.to_string();
                    break;
                }
            }
        }
    }

    let sql_ident;
    if args.len() == 1 {
        if mybatis_name.is_empty() {
            panic!("[mybatis] you should add mybatis ref param  rb:&Mybatis  or rb: &mut MybatisExecutor<'_,'_>  on '{}()'!", target_fn.sig.ident);
        }
        sql_ident = args.get(0).expect("[mybatis] miss pysql sql param!").to_token_stream();
    } else if args.len() == 2 {
        mybatis_ident = args.get(0).expect("[mybatis] miss mybatis ident param!").to_token_stream();
        mybatis_name = format!("{}", mybatis_ident);
        sql_ident = args.get(1).expect("[mybatis] miss pysql sql param!").to_token_stream();
    } else {
        panic!("[mybatis] Incorrect macro parameter length!");
    }

    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    if !is_async {
        panic!(
            "[rbaits] #[crud_table] 'fn {}({})' must be  async fn! ",
            func_name_ident, func_args_stream
        );
    }
    //append all args
    let sql_args_gen = filter_args_context_id(&mybatis_name, &get_fn_args(target_fn));
    let is_fetch = is_fetch(&return_ty.to_string());
    let mut call_method = quote! {};
    if is_fetch {
        call_method = quote! {
             use mybatis::executor::{Executor,ExecutorMut};
             #mybatis_ident.fetch(&sql,rb_args).await
        };
    } else {
        call_method = quote! {
             use mybatis::executor::{Executor,ExecutorMut};
             #mybatis_ident.exec(&sql,rb_args).await
        };
    }
    if return_ty.to_string().contains("Page <")
        && func_args_stream.to_string().contains("& PageRequest")
    {
        let page_ident = get_page_req_ident(target_fn, &func_name_ident.to_string());
        call_method = quote! {
            use mybatis::crud::{CRUD,CRUDMut};
            #mybatis_ident.fetch_page(&sql,rb_args,#page_ident).await
        };
        println!("gen return");
    }

    //gen rust code templete
    return quote! {
       pub async fn #func_name_ident(#func_args_stream) -> #return_ty {
         let mut sql = #sql_ident.to_string();
         let mut rb_arg_map = rbson::Document::new();
         #sql_args_gen
         #fn_body
         use mybatis::executor::{MybatisRef};
         let driver_type = #mybatis_ident.get_mybatis().driver_type()?;
         use mybatis::{mybatis_sql,AsSqlTag};
         let sql_tag = driver_type.sql_tag();
         #[rb_py(#sql_ident)]
         pub fn #func_name_ident(arg: &rbson::Bson, _tag: char) {}
         let (mut sql,rb_args) = #func_name_ident(&rbson::Bson::Document(rb_arg_map), sql_tag);
         driver_type.do_replace_tag(&mut sql);
         #call_method
       }
    }
        .into();
}


pub(crate) fn filter_args_context_id(
    mybatis_name: &str,
    fn_arg_name_vec: &Vec<String>,
) -> proc_macro2::TokenStream {
    let mut sql_args_gen = quote! {};
    for item in fn_arg_name_vec {
        let item_ident = Ident::new(&item, Span::call_site());
        let item_ident_name = item_ident.to_string();
        if item.eq(&mybatis_name) {
            continue;
        }
        sql_args_gen = quote! {
             #sql_args_gen
             rb_arg_map.insert(#item_ident_name.to_string(),rbson::to_bson(#item_ident).unwrap_or_default());
        };
    }
    sql_args_gen
}
