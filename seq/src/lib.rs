/*** 
 * @Author: plucky
 * @Date: 2022-07-17 17:51:13
 * @LastEditTime: 2022-07-22 16:31:04
 * @Description: 
 */

mod seq_parser;



use proc_macro::TokenStream;

use seq_parser::SeqParser;
use syn::parse_macro_input;


#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    //let st = syn::parse_macro_input!(input as DeriveInput);
    let seq = parse_macro_input!(input as SeqParser);
    // println!("{:#?}", seq);

    seq.generate().into()
    
    //compile_error!("seq macro is not yet implemented");

    //  "".parse().unwrap()

}
