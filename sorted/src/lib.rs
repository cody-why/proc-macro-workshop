/**
 * @Author: plucky
 * @Date: 2022-07-17 17:51:13
 * @LastEditTime: 2022-07-24 18:06:07
 * @Description: 
 */

mod sorted;

use proc_macro::TokenStream;
use quote::ToTokens;
use sorted::{Sorted, Check};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    // 第一个参数代表宏本身#[xxx]，第二个参数代表被修饰的代码块
    let _ = args;
    let item= parse_macro_input!(input as syn::Item);
    

    // 编译错误提示要求是变量的位置，而不是在宏的位置,需要to_compile_error配合使用
    match Sorted::generate(&item) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            // 只有错误代码,没有原始代码
            let mut t = e.to_compile_error();
            // 原始的用户代码也要返回
            t.extend(item.to_token_stream());
            t.into()
        }
    }
        
}

#[proc_macro_attribute]
pub fn check(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 这个过程宏的目标是函数
    let mut item= parse_macro_input!(input as syn::ItemFn);


    match Check::generate(&mut item) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            let mut t = e.to_compile_error();
            // 原始的用户代码也要返回
            t.extend(item.to_token_stream());
            t.into()
        }
        // _ => {},
    }

    // r#"fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f, "{}", 1) }"#.parse().unwrap()
    // item.to_token_stream().into()
}