/***
 * @Author: plucky
 * @Date: 2022-09-06 16:33:59
 * @LastEditTime: 2022-09-07 16:29:53
 * @Description: 
 */
#![allow(unused)]


/// #[name(key)] 是否有name属性key
pub fn have_attribute(attrs: &Vec<syn::Attribute>, name: &str, key: &str) -> bool {
    for attr in attrs.iter() {
        if !attr.path.is_ident(name){
            continue;
        }
        if let Ok(list) = attr.parse_meta(){
            if let syn::Meta::List(list) = list {
                for nested in list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested {
                        if path.is_ident(key) {
                            return true;
                        }
                        
                    }
                }
            }

        }
          
    }
    false
}


/// #[name(key="val")] 获取name属性key的值
pub fn get_attribute_name_key(attrs: &Vec<syn::Attribute>, name: &str, key: &str) -> String {
    match attrs.iter()
        .find(|a| a.path.is_ident(name))
        .map(|a| a.parse_meta())
        {
            Some(Ok(syn::Meta::List(list))) => {
                for nested in list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested {
                        if name_value.path.is_ident(key) {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                return lit_str.value();
                            }
                        }
                    }
                }

            }
            _ => {}
        };
    "".into()
}


/// #[debug = "0b{:08b}"] 取出属性debug的值
pub fn get_attribute_key(attrs: &Vec<syn::Attribute>, name: &str)-> Option<String>{
    for attr in attrs {
        // Meta是NameValue(MetaNameValue)的属性
        if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
            ref path,
            ref lit,
            ..
        }))=attr.parse_meta() {
            if path.is_ident(name) {
                if let syn::Lit::Str(ref s) = lit {
                    return Some(s.value());
                }
            }
        }

    }
    None
}
