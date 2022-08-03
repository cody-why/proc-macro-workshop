/**
 * @Author: plucky
 * @Date: 2022-07-17 17:58:28
 * @LastEditTime: 2022-07-27 00:02:31
 * @Description: 
 */


mod builder;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", input);

    let context = builder::BuilderContext::new(input);
    let expanded = context.generate();
    TokenStream::from(expanded)

}

