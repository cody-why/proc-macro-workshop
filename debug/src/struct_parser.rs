/*** 
 * @Author: plucky
 * @Date: 2022-07-15 21:42:52
 * @LastEditTime: 2022-07-21 18:40:49
 * @Description: 
 */



use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Field, Ident, Result, Token, Generics};

// Parses a struct with attributes.
//
//  pub struct S{}
#[allow(dead_code)]
#[derive(Debug)]
pub struct StructParser {
    /// 属性
    pub attrs: Vec<Attribute>,
    /// 可见性
    pub vis: syn::Visibility,
    /// "struct"
    struct_token: Token![struct],
    /// 结构体名称
    pub name: Ident,
    pub generics: Generics,
    pub where_clause: Option<syn::WhereClause>,
    /// {}
    brace_token: syn::token::Brace,
    /// 字段
    pub fields: Punctuated<Field, Token![,]>,
}

impl Parse for StructParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        
        Ok(StructParser {
            attrs: input.call(Attribute::parse_outer)?,
            vis:input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            generics: input.parse::<Generics>()?,
            where_clause: input.parse()?,
            brace_token: syn::braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}

/// 从DeriveInput中解析出fields
#[allow(dead_code)]
fn get_fields_by_input(input: syn::DeriveInput)-> Punctuated<Field, Token![,]>{
    // input.data枚举是Struct(DataStruct)中的fields枚举是Named(FieldsNamed)中的named
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed {named, .. }),
            ..
        }) => named,
        _ => panic!("Only structs are supported"),
    };
    fields
}

/// Meta是NameValue(MetaNameValue)的属性 e.q #[debug = "0b{:08b}"] 取出debug的值
#[allow(dead_code)]
pub fn get_attribute_of_field(attrs: &Vec<Attribute>, name: &str)-> Option<String>{
    for attr in attrs {
        if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
            ref path,
            ref lit,
            ..
        }))=attr.parse_meta() {
            // e.q 'debug'
            if path.is_ident(name) {
                if let syn::Lit::Str(ref s) = lit {
                    return Some(s.value());
                }
            }
        }

    }
    None
}


#[allow(dead_code)]
/// Meta是List(MetaList)的属性 e.q #[debug(bound = "T::Value: Debug")] 取出bound的值
pub fn get_attribute_of_struct(attrs: &[Attribute], name: &str) -> Option<String> {
    if let Some(attr) = attrs.last() {
        if let Ok(syn::Meta::List(syn::MetaList { nested, .. })) = attr.parse_meta() {
            if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(path_value))) = nested.last() {
                if path_value.path.is_ident(name) {
                    if let syn::Lit::Str(ref lit) = path_value.lit {
                        return Some(lit.value());
                    }
                }
            }
        }
    }
    None
}