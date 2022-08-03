/**
 * @Author: plucky
 * @Date: 2022-08-01 16:10:47
 * @LastEditTime: 2022-08-03 20:58:31
 * @Description: 
 */

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, parse_macro_input};
use quote::{quote};


// 实现枚举的Specifier trait,成员的值是bits位数
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut bits = 0usize;
    let mut unit_type = quote!{};
  
    let mut from_bytes_arms = Vec::new();
    let mut discriminant_in_range_check = Vec::new();
    if let syn::Data::Enum(syn::DataEnum{ref variants,..}) = input.data{
        from_bytes_arms = variants.iter().map(|variant|{
          let ident = &variant.ident;
          //_x if _x == MyEnum::A as i32 => MyEnum::A,
          quote!{
            _x if _x == Self::#ident as <Self as Specifier>::UNIT => {
                Self::#ident
            }
          }
        }).collect();
    
        // 8
        let count_variants = variants.iter().count();
        if !count_variants.is_power_of_two(){
            let message = "BitfieldSpecifier expected a number of variants which is a power of 2";
            let error_token = syn::Error::new(Span::call_site(),message).into_compile_error();
            return TokenStream::from(error_token);
        }

        let b = count_variants.next_power_of_two().trailing_zeros() as usize;
        if b > bits{
            bits = b;
            // println!("bits: {}", bits);
            unit_type = match bits{
                0..=8 => quote!{u8},
                9..=16 => quote!{u16},
                17..=32 => quote!{u32},
                33..= 64 => quote!{u64},
                _ => unreachable!() 
            };
        }

        // 9 discriminant
        discriminant_in_range_check = variants.iter().map(|variant|{
          let ident = &variant.ident;
          let span = ident.span();
          quote::quote_spanned!(span => 
            impl ::bitfield::check::CheckDiscriminantInRange<[(); Self::#ident as usize]> for #name{
              type CheckType = [(); ((Self::#ident as usize) < (0x01_usize << #bits)) as usize];
            }
          )
        }).collect();
    }

    let expanded = quote!{
        #(#discriminant_in_range_check)*

        impl Specifier for #name{
            const BITS:usize = #bits; // 占用的位数
            type UNIT = #unit_type; // 关联的储存类型
            type InOut = #name; // 关联的输入输出类型,这里是枚举类型

            // 将输入枚举值转换为对应存储的值
            fn to_bytes(input: Self::InOut) -> Self::UNIT{
                input as Self::UNIT
            }

            // 将对应存储的数值转换为枚举值
            fn try_from(v: Self::UNIT) -> Self::InOut{
                match v {
                    #(#from_bytes_arms),*
                    _ => unreachable!()
                } 
            }
        }
    };
   
    TokenStream::from(expanded)
}