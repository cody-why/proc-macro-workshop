/** 
 * @Author: plucky
 * @Date: 2022-07-24 22:33:33
 * @LastEditTime: 2022-08-03 22:21:44
 * @Description: 
 */

use proc_macro2::TokenStream;

use syn::{parse_macro_input, ItemStruct, Field};
use quote::{quote, format_ident};


pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item  = parse_macro_input!(input as syn::ItemStruct);
    // println!("{:#?}", item);

    let mut tokens = TokenStream::new();
    tokens.extend(gen_enum_bits());
    tokens.extend(gen_struct(item));
    
    tokens.into()

    
    // println!("{:#?}", tokens);
    // "".parse().unwrap()
}

// 实现类似这样的trait
// pub trait Specifier {
//     const BITS: usize;
// }
// enum B24 {}
// impl Specifier for B24 {
//     const BITS: usize = 24;
// }
fn gen_enum_bits() -> TokenStream {
    // B1..B32
    let iter = (1..=32usize).into_iter().map(|i|{
        let name = format_ident!("B{}", i as u32);
        
        let n = if i<9{
            8
        }else if i<17{
            16
        }else if i<33{
            32
        }else{
            64
        };
        // 指定类型
        let unit = format_ident!("u{}", n as u32);

        quote!{
            pub enum #name {}
            impl Specifier for #name {
                const BITS:usize = #i;
                type UNIT = #unit;
                type InOut = #unit;

                fn to_bytes(input: Self::InOut) -> Self::UNIT {
                    input
                }
                fn try_from(v: Self::UNIT) -> Self::InOut {
                    v
                }

            }
        }
        
    });
    
    quote!{
        
        // 展开B1..B32
        #(#iter)*

        // bool 的 Specifier
        impl Specifier for bool {
            const BITS:usize= 1;
            type UNIT = u8;
            type InOut = bool;

            fn to_bytes(input: bool) -> u8 {
                input as u8
            }
            fn try_from(v: u8) -> bool{
                v == 1
            }
        }

    }

}

// 构造 stuct 
fn gen_struct(item: ItemStruct) -> TokenStream {

    let fields = item.fields.into_iter().collect::<Vec<_>>();
    let get_set = gen_get_set(&fields);
   
    let name = item.ident;
    let vis = item.vis;

    let mut bits = quote!{0};
    fields.iter().for_each(|f| {
        let ty = &f.ty;
        bits = quote!{<#ty as Specifier>::BITS + #bits};
    });

    let bits = quote!{((#bits-1)/8+1) as usize};
    // println!("{}", bits);
    quote!{

        #[repr(C)]
        #vis struct #name {
            data: [u8; #bits],
            // #(#fields),*
            
        }

        impl #name {
            pub fn new() -> Self {
                Self {
                    data: [0; #bits],                  
                    
                }
            }
            #(#get_set)*
            
        }
    }
}


#[allow(dead_code)]
// getters setters
fn gen_get_set<'a>(fields:&'a Vec<Field>) -> impl Iterator<Item= TokenStream>+ 'a{

    let mut start_bit = quote!{0};
    
    fields.iter().map(move|f|{
        let name = f.ident.as_ref().unwrap();
        let getter = format_ident!("get_{}", name);
        let setter = format_ident!("set_{}", name);
        let ty = &f.ty;

        // let specifier = quote!{<#ty as Specifier>};
        let bits = quote!{<#ty as Specifier>::BITS};
        let start = quote!{#start_bit};
        start_bit = quote!{#start_bit + #bits};
        
        let unit = quote!{<#ty as Specifier>::UNIT};
        let inout = quote!{<#ty as Specifier>::InOut};

        quote! {
            pub fn #setter(&mut self, v: #inout) -> &mut Self {
                let start = (#start) as usize;
                let mut index = start / 8;// 数组索引
                let mut offset = start % 8;// 位索引
                let mut bits = (#bits) as usize; // 剩余copy位数

                let mut value = <#ty as Specifier>::to_bytes(v);
               
                while bits > 0 {
                     // 这次能够copy的最小位数
                    let copy_bits = std::cmp::min(bits, 8-offset);
                    let end = offset + copy_bits;
                    let b = value as u8;
                    self.data[index].set_bits(offset..end, b);
                    value >>= copy_bits;//高位的值在下个u8中
                    bits -= copy_bits;
                    offset += copy_bits;
                    if offset >= 8 {
                        offset = 0;
                        index += 1;
                    }
                }
                
                self
            }

            pub fn #getter(&mut self) -> #inout {
                let start = (#start) as usize;
                let mut index = start / 8;// 数组索引
                let mut offset = start % 8;// 位索引
                
                let mut bits = (#bits) as usize; // 剩余copy位数
                
                let mut value = false as #unit;
                let mut last_copy = 0;

                while bits > 0 {
                    // 这次能够copy的最小位数
                    let copy_bits = std::cmp::min(bits, 8-offset);
                    let end = offset + copy_bits;
                    let b = self.data[index].get_bits(offset..end);
                    value |= (b as #unit) << last_copy;//后面的值放在高位
                    bits -= copy_bits;
                    offset += copy_bits;
                    if offset >= 8 {
                        offset = 0;
                        index += 1;
                    }
                    last_copy += copy_bits;
                }
                // println!("{}", value);
                <#ty as Specifier>::try_from(value)
                                
            }
        }

        
    })


}
