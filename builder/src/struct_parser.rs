/*** 
 * @Author: plucky
 * @Date: 2022-07-15 21:42:52
 * @LastEditTime: 2022-07-17 01:12:46
 * @Description: 
 */



use std::collections::HashMap;


use proc_macro2::{TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Field, Ident, Result, Token};

// Parses a struct with attributes.
//
//  pub struct S{}
#[allow(dead_code)]
#[derive(Debug)]
pub struct StructParser {
    /// 可见性
    vis: syn::Visibility,
    /// 属性
    attrs: Vec<Attribute>,
    /// "struct"
    struct_token: Token![struct],
    /// 结构体名称
    name: Ident,
    /// {}
    brace_token: syn::token::Brace,
    /// 字段
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for StructParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(StructParser {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            brace_token: syn::braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}


impl StructParser {
    pub fn show_attributes(&self) {
        for field in &self.fields {
            // println!("{:?}", field);

            field.attrs.iter().for_each(|attr| {
                // 输出属性名称builder
                println!("{:?}", attr.path.get_ident().unwrap());
                // TokenStream有个TokenTree的IntoIterator实现，可以用来遍历TokenTree
                let tokens = attr.tokens.clone();
                tokens.into_iter().for_each(|t| {
                    match t {
                        // 第一个节点是Group,stream()获取其中的TokenStream,再进行IntoIterator遍历
                        TokenTree::Group(g) => {
                            let mut ident = vec![];
                            let mut literal = vec![];
                            g.stream().into_iter().for_each(|f|{
                                match f {
                                    TokenTree::Ident(i) =>ident.push(i.to_string()),//each
                                    // TokenTree::Literal(l) => literal.push(l.to_string().replace("\"", "")),//"env"
                                    TokenTree::Literal(l) => literal.push(l.to_string()),//"env"
                                    
                                    _ => {},
                                }
                            });
                           
                            let map = ident.iter().zip(literal.iter()).collect::<HashMap<_,_>>();
                            println!("{:?}", map);
                        }
                        _ => panic!("not supported"),
                    }
                });
            });

            
        }
    }
}
