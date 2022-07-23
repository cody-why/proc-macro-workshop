/**
 * @Author: plucky
 * @Date: 2022-07-17 17:51:13
 * @LastEditTime: 2022-07-24 00:28:00
 * @Description: 
 */

mod sorted;

use proc_macro::TokenStream;
use quote::ToTokens;
use sorted::Sorted;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    // 第一个参数代表宏本身#[xxx]，第二个参数代表被修饰的代码块
    let _ = parse_macro_input!(args as syn::AttributeArgs);
    let item= parse_macro_input!(input as syn::Item);
    

    // 编译错误提示要求是变量的位置，而不是在宏的位置,需要to_compile_error配合使用
    match Sorted::generate(&item) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            let mut t = e.to_compile_error();
            // 原始的用户代码也要返回
            t.extend(item.to_token_stream());
            t.into()
        }
    }
        
}
