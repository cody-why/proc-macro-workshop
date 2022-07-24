/**
 * @Author: plucky
 * @Date: 2022-07-21 20:35:25
 * @LastEditTime: 2022-07-24 18:47:39
 * @Description: 
 */

use proc_macro2::{TokenStream, TokenTree};
use syn::parse::{Parse, ParseStream, Result};

use syn::{Ident, Token, LitInt};
use quote::quote;

/// A parser for the `seq` macro.
#[derive(Debug)]
pub struct SeqParser {
    pub name: Ident,
    //pub ty: Type,//Expr, 
    pub start: usize,
    pub end: usize,
    pub body: TokenStream,
}

/// 实现 Parse trait
impl Parse for SeqParser {
 
    fn parse(input: ParseStream) -> Result<Self> {
        // N in 0..8 展开分别是
        // syn::Ident, Token![in], syn::LitInt,Token![..], syn::LitInt.
        let name: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start: LitInt = input.parse()?;
        input.parse::<Token![..]>()?;

        // 第7关要求是inclusive range，所以查看如果有=,end值加1
        let mut inclusive = false;
        if input.peek(Token!(=)) {
            input.parse::<Token!(=)>()?;
            inclusive = true;
        }

        let end: LitInt = input.parse()?;
        let mut end = end.base10_parse::<usize>()?;
        if inclusive  {
            end += 1;
        }

        let content;
        syn::braced!(content in input);
        
        Ok(Self {
            name,
            start:start.base10_parse()?,
            end,
            body:content.parse()?,
        })
        
    }
}

impl SeqParser {
    pub fn generate(&self) -> TokenStream {
        // eprintln!("{:#?}", self);

        let mut token = proc_macro2::TokenStream::new();

        // 用字符串拼接的方式,作者不希望这么简单粗暴吧.
        // let body = &self.body.to_string();
        // for i in self.start..self.end {            
            // let a = body.replace(&self.name.to_string(), i.to_string().as_str());
            // let t:TokenStream = a.parse().unwrap();
            // token.extend(t);
        // }
        
        let (find ,stream)= self.gen_body_match_cycle(&self.body, self.start, self.end);
        if find {
            token.extend(stream);
            // println!("{:#?}", token);
            
            return token;
        }

        for i in self.start..self.end {
            token.extend(self.gen_body(&self.body, i));

        }
        token

        // println!("{:#?}", token);
        // "".parse().unwrap()

    }
    
    /// 展开body,替换stringify!(N)里的'N'为n
    pub fn gen_body(&self, body: &TokenStream, n: usize) -> TokenStream {
        let mut stream = TokenStream::new();
     
        // 用索引方便遍历到想要的index
        let buf = body.clone().into_iter().collect::<Vec<_>>();
        let mut idx = 0;
        let len = buf.len();
        while idx < len {
            idx+=1;
            let i = idx-1;
            let t = &buf[i];
            // println!("{:?}\n",t);
            match t {
                // 如果是Group,则递归展开内部的TokenStream
                TokenTree::Group(g) => {
                    let ts = self.gen_body(&g.stream(), n);
                    let g = proc_macro2::Group::new(g.delimiter(), ts);
                    stream.extend(quote!{#g});
                },
                // Ident是N,则替换为n
                TokenTree::Ident(ident) => {
                    if ident == &self.name {
                        // 数值为Literal
                        let lit = proc_macro2::Literal::usize_unsuffixed(n as usize);
                        stream.extend(quote!{#lit});
                        continue;
                    }
                    // 将Ident'f' Punct'~' Ident'N' 替换为 Ident'f1'
                    // 如果是f~N,则替换为f1,f2,f3...
                    // 思路:判断i+1是不是'~',i+2是不是N,i+3是不是'~'
                    if i+1 < len && &buf[i+1].to_string() == "~"{
                        // 如果是f~N,则要把f和1拼接起来组成新的ident
                        // 第4关可选的情况,如果是f~N~_suffix,则替换为f1_suffix...
                        if i+4 < len && &buf[i+3].to_string() == "~" {
                            let fn_name = format!("{}{}{}", ident, n, &buf[i+4]);
                            let ni = proc_macro2::Ident::new(&fn_name, ident.span());
                            stream.extend(quote!{#ni});
                            idx += 4; // 前进4步,~ N ~ _suffix
                        }else{
                            let fn_name = format!("{}{}", ident, n);
                            let ni = proc_macro2::Ident::new(&fn_name, ident.span()); // proc_macro2::Span::call_site()
                            stream.extend(quote!{#ni});
                            idx += 2; // 前进2步,~ N
                        }
                        
                        continue;
                    }
                   
                    stream.extend(quote!{#ident});
                },
                
                // 其他的TokenTree直接返回
                _ => stream.extend(quote!{#t}),
            }
            
        }

        stream
        
        
    }
   
    /// 匹配#(XXXN)*的情况
    #[allow(dead_code)]
    pub fn gen_body_match_cycle(&self, body: &TokenStream, start:usize,end:usize) ->(bool, TokenStream) {
        let mut stream = TokenStream::new();
        let mut find = false;
        // 用索引方便遍历到想要的index
        let buf = body.clone().into_iter().collect::<Vec<_>>();
        let mut idx = 0;
        let len = buf.len();

        while idx < len {
            idx+=1;
            let i = idx-1;
            let t = &buf[i];
            // println!("{:?}\n",t);
            match t {
                // #(XXXN)* 在Group里,递归展开内部的TokenStream
                TokenTree::Group(g) => {
                    let (f, ts) = self.gen_body_match_cycle(&g.stream(), start, end);
                    if f {
                        find = f;
                    } 
                    let g = proc_macro2::Group::new(g.delimiter(), ts);
                    stream.extend(quote!{#g});
                },
                // 匹配#(XXXN)*的情况,Punct# Group Punct*
                TokenTree::Punct(p) => {
                    if p.as_char() == '#' && i+2<len{
                        // 如果是#,则需要判断是不是匹配 Group和*
                        if &buf[i+2].to_string() == "*" {
                            // 如果是#(XXXN)*,内部是Group
                            if let TokenTree::Group(g) = &buf[i+1]{
                                for n in start..end {
                                    let ts = self.gen_body(&g.stream(), n);
                                    stream.extend(quote!{#ts});
                                }
                                idx+=2; // 前进3步,# ( XXXN ) *
                                find = true;
                                continue;
                            }
                        }
                    }
                    // 不要忽略其他的#
                    stream.extend(quote!{#t});
                },
                
                
                // 其他的TokenTree直接返回
                _ => stream.extend(quote!{#t}),
            }
            
        }


        (find, stream)
    }

}

