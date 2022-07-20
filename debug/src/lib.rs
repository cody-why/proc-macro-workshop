/*** 
 * @Author: plucky
 * @Date: 2022-06-07 07:11:48
 * @LastEditTime: 2022-07-20 23:08:27
 * @Description: 
 */

mod struct_parser;
mod builder;
mod type_path;

use proc_macro::TokenStream;
use syn::{parse_macro_input};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {

    let sp = parse_macro_input!(input as struct_parser::StructParser);
    //  println!("{:#?}", sp);
    let context = builder::BuilderContext::new(sp);
    let expanded = context.generate();
    TokenStream::from(expanded)

    //  "".parse().unwrap()

}
