/*
 * @Author: plucky
 * @Date: 2022-09-06 16:33:59
 * @LastEditTime: 2022-10-22 21:41:27
 * @Description: 
 */

#![allow(unused)]

/// `#[name(value)]` attribute value exist or not
pub fn has_attribute_value(attrs: &Vec<syn::Attribute>, name: &str, value: &str) -> bool {
    for attr in attrs.iter() {
        if !attr.path.is_ident(name){
            continue;
        }
        if let Ok(list) = attr.parse_meta(){
            if let syn::Meta::List(list) = list {
                for nested in list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested {
                        if path.is_ident(value) {
                            return true;
                        }
                        
                    }
                }
            }

        }
          
    }
    false
}

/// `#[name]` attribute name exist or not
pub fn has_attribute(attrs: &Vec<syn::Attribute>, name: &str) -> bool {
    // for attr in attrs.iter() {
    //     if attr.path.is_ident(name){
    //         return  true;
    //     }
    // }
    // false

    attrs.iter().any(|attr| attr.path.is_ident(name))
}

/// `#[name(key="val")]` Get the value of the name attribute by key
pub fn get_attribute_by_key(attrs: &Vec<syn::Attribute>, name: &str, key: &str) -> Option<String> {
    match attrs.iter()
        .find(|a| a.path.is_ident(name))
        .map(|a| a.parse_meta())
        {
            Some(Ok(syn::Meta::List(list))) => {
                for nested in list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested {
                        if name_value.path.is_ident(key) {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                return Some(lit_str.value());
                            }
                        }
                    }
                }

            }
            _ => {}
        };
    None
}


/// `#[name = "0b{:08b}"]` Get the value of the name attribute
pub fn get_attribute_value(attrs: &Vec<syn::Attribute>, key: &str)-> Option<String>{
    for attr in attrs {
        // Meta是NameValue(MetaNameValue)的属性
        if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
            ref path,
            ref lit,
            ..
        }))=attr.parse_meta() {
            if path.is_ident(key) {
                if let syn::Lit::Str(ref s) = lit {
                    return Some(s.value());
                }
            }
        }

    }
    None
}

/// whether `Option<inner_type>` returns (whether Option, inner_type).
pub fn get_option_type(ty: &Type) -> (bool, &Type){
    get_inner_type(ty, "Option")
}

#[allow(dead_code)]
/// whether `Vec<inner_type>` returns (whether Vec, inner_type).
pub fn get_vec_type(ty: &Type) -> (bool, &Type){
    get_inner_type(ty, "Vec")
}


/// whether inner_type,such as: Option<String>,Vec<String>
/// returns (whether, inner_type).
fn get_inner_type<'a>(ty: &'a Type, name:&str) -> (bool, &'a Type) {
    // syn::Type::Path(ref path) {segments[0].ident="Option"}
    if let syn::Type::Path(ref path) = ty{
        if let Some(segment) = path.path.segments.first() {
            if segment.ident == name {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(ty)) = args.first() {
                        return (true, ty);
                        
                    }
                }

            }
        }
    }
    (false, ty)
}