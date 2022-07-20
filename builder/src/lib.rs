/*** 
 * @Author: plucky
 * @Date: 2022-06-07 07:11:48
 * @LastEditTime: 2022-07-18 14:10:40
 * @Description: 
 */


mod builder;
mod struct_parser;

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

#[proc_macro_derive(Builder2, attributes(builder))]
pub fn derive2(input: TokenStream) -> TokenStream {
    let sp = parse_macro_input!(input as struct_parser::StructParser);
    //  println!("{:#?}", sp);
    sp.show_attributes();

    "".parse().unwrap()
}