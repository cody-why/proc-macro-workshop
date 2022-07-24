/** 
 * @Author: plucky
 * @Date: 2022-07-24 22:33:33
 * @LastEditTime: 2022-07-25 00:22:22
 * @Description: 
 */

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use quote::quote;


pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item  = parse_macro_input!(input as syn::ItemStruct);
    // println!("{:#?}", item);

    let mut tokens = TokenStream::new();
    tokens.extend(gen_enum_bits());
    tokens.extend(item.to_token_stream());
    
    tokens.into()

    
    // println!("{:#?}", tokens);
    // "".parse().unwrap()
}

// 实现类似这样的trait
// pub trait Specifier {
//     const BITS: u8;
// }
// enum B24 {}
// impl Specifier for B24 {
//     const BITS: u8 = 24;
// }
fn gen_enum_bits() -> TokenStream {
    // 只产生了24个
    let iter = (1..=24).into_iter().map(|i|{
        let str = format!("B{}", i);
        let name  = syn::Ident::new(&str, proc_macro2::Span::call_site());

        quote!{
            pub enum #name {}
            impl Specifier for #name {
                const BITS: u8 = #i as u8;
            }
        }
        
    });
    
    quote!{
        pub trait Specifier {
            const BITS: u8;
        }
        #(#iter)*

    }

}