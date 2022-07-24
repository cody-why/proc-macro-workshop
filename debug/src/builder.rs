/*** 
 * @Author: plucky
 * @Date: 2022-07-17 18:14:47
 * @LastEditTime: 2022-07-21 18:18:55
 * @Description: 
 */

use std::{iter::Map, slice::Iter, collections::HashMap};

use proc_macro2::{Ident, TokenStream};
use syn::{Type, Generics, Path, TypePath, visit::Visit, Attribute};

use crate::{struct_parser::*, type_path::TypePathVisitor};

type TokenStreamIter<'a> = Map<Iter<'a, Field>, fn(&'a Field) -> TokenStream>;


// 属性 #[debug = "arg"]
// struct Opts {
//     debug: Option<String>,
// }


/// Punctuated<Field, Token![,]> 中的Field和属性宏
#[allow(dead_code)]
struct Field {
    name: Ident,
    ty: Type,
    // #[debug = "arg"]
    format: Option<String>,
    // #[debug(bound = "T::Value: Debug")]
    bound: Option<String>,
}

// lib.rs只写宏定义,builder.rs写宏实现
pub struct BuilderContext {
    // 属性 #[debug(bound = "T::Value: Debug")]
    attrs: Vec<Attribute>,
    // 结构名字
    name: Ident,
    // fields: Punctuated<Field, Token![,]>,
    // 泛型
    generics: Generics,
    // 字段
    fields: Vec<Field>,
}

impl BuilderContext {
    /// Create a new builder context with Parser.
    pub fn new(input: StructParser) -> Self {
        // println!("attrs={:#?}", input.attrs);
        let fields = input.fields;
        let fields = fields.into_iter().map(|f|{
            // println!("field={:#?}", f);
            let format = get_attribute_of_field(&f.attrs, "debug");
            // println!("debug={:?}", format);
            let bound = get_attribute_of_struct(&f.attrs, "bound");
            // println!("bound={:?}", bound);
            
            Field {
                name: f.ident.unwrap(),
                ty: f.ty,
                format,
                bound,
            }
        }).collect();

        let name = input.name;
        let generics = Generics {
            where_clause: input.where_clause,
            ..input.generics
        };

        Self {attrs:input.attrs, name, fields ,generics}

    }

    /// Generate the builder implementation.
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let name_str = name.to_string();
        let fields= self.gen_fields();
        
        // 把每个泛型参数T加上Debug限定
        let generics = self.add_trait_bounds_2();
        
        let (impl_generics, ty_generics, where_clause) = &generics.split_for_impl();
        // println!("where_clause={:?}", where_clause);
       
        quote::quote! {
            //where T:std::fmt::Debug
            impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(#name_str)
                    // 展开所有字段
                     #(#fields)*

                    .finish()
                }
            }
        }
    
        // let body = TokenStream::new();
        // body.extend(quote::quote! {.field(#name_str, &self.#name) });

    }
    

/// 返回一个iter, e.q .field("name","F")
fn gen_fields(&self) -> TokenStreamIter {
    self.fields.iter().map(|f| {
        let name = &f.name;
        let name_str = name.to_string();
        // 属性指定格式化
        if let Some(fmt) = &f.format {
           return quote::quote! {
                // .field(#name_str, &format_args!("{:?}", self.#name))
                .field(#name_str, &format_args!(#fmt, self.#name))
           }
        }
        quote::quote! {
            // 这里的self是代码,不是变量
            .field(#name_str, &self.#name)    
        }  
    })
}

/// 把所有T泛型增加Debug限定
// Add a bound `T: Debug` to every type parameter T.
fn add_trait_bounds_2(&self) -> Generics {
    let mut generics = self.generics.clone();
    // 5
    // PhantomData的泛型参数,T
    let mut phantom_vec:Vec<String> = Vec::new();
    // 结构字段的类型名称
    let mut type_name_vec:Vec<String> = Vec::new();

    let mut bound_map = HashMap::new();
    //遍历所有feild,获取类型的名称和类型是PhantomData的泛型
    self.fields.iter().for_each(|f|{
       if let Some(n) =get_phantom_type(f).unwrap_or(None) {
            phantom_vec.push(n);
       }
       if let Some(n) =get_field_type_name(f).unwrap_or(None) {
            type_name_vec.push(n);
       }
       // 8 bound属性的值
       if let Some(b) = &f.bound{
              bound_map.insert(b,());
       }
       
    });
    
    // 7
    let associated_types = get_generic_associated_types(&generics, &self.fields);
    
    // 8 取得bound = "T::Value: Debug"
    let bound = get_attribute_of_struct(&self.attrs, "bound");
    if let Some(v) = bound {
        // 第8关,如果属性指定限定类型,只需要在where子句中加上限定类型即可   
        generics.make_where_clause();
        let clause = syn::parse_str(v.as_str()).unwrap();
        generics.where_clause.as_mut().unwrap().predicates.push(clause);
        
        return generics;
    }
    
    bound_map.iter().for_each(|(k,_)|{
        // 第8关,如果属性指定限定类型,只需要在where子句中加上限定类型即可   
        generics.make_where_clause();
        let clause = syn::parse_str(k.as_str()).unwrap();
        generics.where_clause.as_mut().unwrap().predicates.push(clause);
        
    });
    if !bound_map.is_empty() {
        return generics;
    }

    // 遍历所有泛型参数,加上Debug限定
    for param in generics.params.iter_mut() {
        if let syn::GenericParam::Type(ref mut t) = param {
            // 第5关,如果泛型T是PhantomData类型,而且其他字段的没有用到它,则不需要加上Debug限定
            let name = t.ident.to_string();
            if phantom_vec.contains(&name) && !type_name_vec.contains(&name) {
                // println!("type_name_vec {:?}", name);
                continue
            }
            // 第7关,如果是关联类型,则需要在where子句中加T::Value的Debug限定,不能在T加限定
            if associated_types.contains_key(&name) && !type_name_vec.contains(&name){
                continue
            }

            t.bounds.push(syn::parse_quote!(std::fmt::Debug));

        }
    }

    // 第7关,如果是关联类型,则需要在where子句中加T::Value的Debug限定
    generics.make_where_clause();
    for (_, types) in associated_types {
        for associated_type in types {
            let clause = syn::parse_quote!(#associated_type:std::fmt::Debug);
            generics.where_clause.as_mut().unwrap().predicates.push(clause);
        }
    }

    generics
}


}

#[allow(dead_code)]
// Add a bound `T: Debug` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        if let syn::GenericParam::Type(ref mut t) = param {
            t.bounds.push(syn::parse_quote!(std::fmt::Debug));
        }
    }
    generics
}

/// 取出PhantomData的泛型参数,T
fn get_phantom_type(f: &Field) ->syn::Result<Option<String>> {
    let mut result = Ok(None);
    // 一个闭包
    let cb = |ident:&Ident, p:&syn::PathArguments|{
        if ident == "PhantomData" {
            if let syn::PathArguments::AngleBracketed(ref angle_bracketed) = p {
                for arg in angle_bracketed.args.iter() {
                    if let syn::GenericArgument::Type(ref ty) = arg {
                        // 一个闭包
                        let cb = |ident:&Ident, _:&syn::PathArguments|{  
                            // println!("PhantomData ident={:?}", ident);
                            result = Ok(Some(ident.to_string()));
                            
                        };
                        get_type_ident(ty, cb);
                        
                    }
                }
            }

        }
        
    };
    get_type_ident(&f.ty, cb);
    result
    
}

/// 取出该类型的Ident,接受一个闭包返回Ident
fn get_type_ident<F>(ty: &Type, mut f: F) where F: FnMut(&Ident,&syn::PathArguments) {
    if let Type::Path(TypePath{path:Path{ref segments, ..}, ..}) = &ty {
        if let Some(segment) = segments.first() {
            let syn::PathSegment{ident, arguments } = segment;
            {
                f(ident, arguments);
            }

            
        }
    }
}

/// 取出结构属性的类型的名称,a:XXX,返回XXX;a:Option<XXX>,返回Option
#[allow(dead_code)]
fn get_field_type_name(field: &Field) -> syn::Result<Option<String>> {
    if let syn::Type::Path(syn::TypePath{path: syn::Path{ref segments, ..}, ..}) = field.ty {
        if let Some(syn::PathSegment{ref ident,..}) = segments.last() {
            return Ok(Some(ident.to_string()))
        }
    }
    Ok(None)
}

/// 取所有泛型参数的关联类型,例如values:Vec<T::Value>, 返回的T::Value是T的关联类型
#[allow(dead_code)]
fn get_generic_associated_types(generics: &Generics, fields:&Vec<Field>) -> HashMap<String, Vec<syn::TypePath>> {
    // 遍历所有泛型参数的名字
    let generic_type_names: Vec<String> = generics.params.iter().filter_map(|f| {
        if let syn::GenericParam::Type(ty) = f {
            return Some(ty.ident.to_string())
        }
        None
    }).collect();

    
    let mut visitor = TypePathVisitor {
        generic_type_names,  // 用筛选条件初始化Visitor
        associated_types: HashMap::new(),
    };
    // 遍历所有的语法树节点
    for f in fields {
        visitor.visit_type(&f.ty);
    }
    // input: &syn::DeriveInput
    // visitor.visit_derive_input(input);
    visitor.associated_types
}