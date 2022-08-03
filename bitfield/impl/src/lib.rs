/**
 * @Author: 
 * @Date: 2022-07-17 17:51:13
 * @LastEditTime: 2022-08-01 16:11:03
 * @Description: 
 */

mod bitfield;
mod tests;
mod bitfield_specifier;


use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    bitfield::expand(input)
}

#[proc_macro_derive(BitfieldSpecifier)]
pub fn derive(input: TokenStream) -> TokenStream {
    bitfield_specifier::derive(input)
}