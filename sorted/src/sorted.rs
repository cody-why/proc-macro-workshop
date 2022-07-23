
/**
 * @Author: plucky
 * @Date: 2022-07-23 19:34:46
 * @LastEditTime: 2022-07-24 00:14:02
 * @Description: 
 */


use proc_macro2::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;



 #[allow(dead_code)]
pub struct Sorted {
    // pub args: syn::AttributeArgs,
    // pub item: syn::ItemEnum,
}

impl Sorted {

    pub fn generate(item: &syn::Item)-> syn::Result<TokenStream>{
        // println!("{:#?}", item);
        
        match item {
            syn::Item::Enum(e) => {
                check_order(e)
                
            },   
            _ => {return Err(syn::Error::new(Span::call_site(), "expected enum or match expression"))},
        }

    }

    

}

/// 比较枚举的每个variant的名称是否按照字典序排列
fn check_order(item: &syn::ItemEnum)-> syn::Result<TokenStream>{
        let mut i = 0;
        let origin = item.variants.iter().map(|f|{&f.ident}).collect::<Vec<_>>();
        let mut sorted =  origin.clone();
        // 排序ident
        sorted.sort_by(|a,b|{
            let a_str = a.to_string();
            let b_str = b.to_string();
            a_str.cmp(&b_str)

        });
        // println!("{:#?}", sorted);
        for v in origin {
            if v != sorted[i] {
                let str = format!("{} should sort before {}", sorted[i], v );
                return Err(syn::Error::new(sorted[i].span(), str));
            }
            i += 1;
        }

        return Ok(item.to_token_stream());
}
    
