/**
 * @Author: plucky
 * @Date: 2022-07-17 17:51:13
 * @LastEditTime: 2022-07-25 00:16:24
 * @Description: 
 */

mod bitfield;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    bitfield::expand(input)
}
