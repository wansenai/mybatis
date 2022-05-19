use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, FnArg, ItemFn};

use crate::proc_macro::TokenStream;
use crate::util::{find_fn_body, find_return_type, get_fn_args, get_page_req_ident, is_fetch, is_mybatis_ref};

//impl mybatis_sql macro
pub(crate) fn impl_macro_mybatis_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let return_ty = find_return_type(target_fn);
    let func_name_ident = target_fn.sig.ident.to_token_stream();

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
        sql_ident = args.get(0).expect("[mybatis] miss mybatis_sql macaro param!").to_token_stream();
    } else if args.len() == 2 {
        mybatis_ident = args.get(0).expect("[mybatis] miss mybatis ident param!").to_token_stream();
        mybatis_name = format!("{}", mybatis_ident);
        sql_ident = args.get(1).expect("[mybatis] miss mybatis_sql macro sql param!").to_token_stream();
    } else {
        panic!("[mybatis] Incorrect macro parameter length!");
    }

    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let fn_body = find_fn_body(target_fn);
    let is_async = target_fn.sig.asyncness.is_some();
    if !is_async {
        panic!(
            "[rbaits] #[crud_table] 'fn {}({})' must be  async fn! ",
            func_name_ident, func_args_stream
        );
    }
    let mut call_method = quote! {};
    let is_fetch = is_fetch(&return_ty.to_string());
    if is_fetch {
        call_method = quote! {fetch};
    } else {
        call_method = quote! {exec};
    }
    //check use page method
    let mut page_req_str = String::new();
    let mut page_req = quote! {};
    if return_ty.to_string().contains("Page <")
        && func_args_stream.to_string().contains("& PageRequest")
    {
        let req = get_page_req_ident(target_fn, &func_name_ident.to_string());
        page_req_str = req.to_string();
        page_req = quote! {,#req};
        call_method = quote! {fetch_page};
    }
    //append all args
    let sql_args_gen = filter_args_context_id(&mybatis_name, &get_fn_args(target_fn), &[page_req_str]);
    //gen rust code templete
    let gen_token_temple = quote! {
       pub async fn #func_name_ident(#func_args_stream) -> #return_ty{
           let mut rb_args =vec![];
           #sql_args_gen
           #fn_body
           use mybatis::executor::{Executor,ExecutorMut};
           return #mybatis_ident.#call_method(&#sql_ident,rb_args #page_req).await;
       }
    };
    return gen_token_temple.into();
}

fn filter_args_context_id(
    mybatis_name: &str,
    fn_arg_name_vec: &Vec<String>,
    skip_names: &[String],
) -> proc_macro2::TokenStream {
    let mut sql_args_gen = quote! {};
    for item in fn_arg_name_vec {
        let item_ident = Ident::new(&item, Span::call_site());
        let item_ident_name = item_ident.to_string();
        if item.eq(&mybatis_name) {
            continue;
        }
        let mut do_continue = false;
        for x in skip_names {
            if x.eq(&item_ident_name) {
                do_continue = true;
                break;
            }
        }
        if do_continue {
            continue;
        }
        sql_args_gen = quote! {
             #sql_args_gen
             rb_args.push(rbson::to_bson(#item_ident).unwrap_or_default());
        };
    }
    sql_args_gen
}
