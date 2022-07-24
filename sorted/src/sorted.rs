
/**
 * @Author: plucky
 * @Date: 2022-07-23 19:34:46
 * @LastEditTime: 2022-07-24 18:45:40
 * @Description: 
 */


use proc_macro2::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::visit_mut::VisitMut;



 #[allow(dead_code)]
pub struct Sorted {}

impl Sorted {
    /// sorted宏代码
    pub fn generate(item: &syn::Item)-> syn::Result<TokenStream>{
        // println!("{:#?}", item);
        
        match item {
            syn::Item::Enum(e) => {
                check_sort_item(e)
                
            },   
            _ => {Err(syn::Error::new(Span::call_site(), "expected enum or match expression"))},
        }

    }

    

}

/// 比较枚举的每个变量的名称是否按照字典序排列
fn check_sort_item(item: &syn::ItemEnum)-> syn::Result<TokenStream>{
        
        let origin = item.variants.iter().map(|f|{&f.ident}).collect::<Vec<_>>();
        let mut sorted =  origin.clone();
        // 排序ident
        sorted.sort_by(|a,b|{
            let a_str = a.to_string();
            let b_str = b.to_string();
            a_str.cmp(&b_str)

        });
        // println!("{:#?}", sorted);
        for (a,b) in origin.iter().zip(sorted.iter()) {
            if a != b {
                let str = format!("{} should sort before {}", b, a );
                return Err(syn::Error::new(b.span(), str));
            }
        }

        Ok(item.to_token_stream())
}
    
pub struct Check {
    err: Option<syn::Error>,
}

/// visit 遍历所有节点
impl VisitMut for Check {
    fn visit_expr_match_mut(&mut self, node: &mut syn::ExprMatch) {
        // println!("{:#?}\n", node);
        for (i,attr) in node.attrs.iter().enumerate() {
            if get_path_string(&attr.path) == "sorted" {
                // 去掉match语句块的sorted属性
                node.attrs.remove(i);
                self.check_sort_arms(node);
                
                if self.err.is_some() {
                    return;
                }
                break; 
            }
        }
        
       
        // Delegate to sub-visitors.
        syn::visit_mut::visit_expr_match_mut(self, node);
    }
}

impl Check {
    fn new()->Self{
        Self { err: None }
    }

    /// check宏代码
    #[allow(dead_code)]
    pub fn generate(item: &mut syn::ItemFn)-> syn::Result<TokenStream>{
        let mut ck = Check::new();
        ck.visit_item_fn_mut(item);
        
        if let Some(e) = ck.err {
            return Err(e);
        }
        Ok(item.to_token_stream())
    }

    /// 检查match语句块的每个arm是否按照字典序排列
    fn check_sort_arms(&mut self, node: &mut syn::ExprMatch){
           let mut sorted = node.arms.clone();

           // 检查有没有不支持的类型
           for a in node.arms.iter() {
                match  get_pat_path(a){
                    Ok(_) => {},
                    Err(e) => {
                        self.err = Some(e);
                        return;
                    },
                }
               
           }

           // 排序arm
            sorted.sort_by(|a, b|{
                let a_str = get_pat_path(a).unwrap();
                let b_str = get_pat_path(b).unwrap();
                a_str.cmp(&b_str)
            });
            // node.arms = sorted;
            // 检查字段是否按字典排序
            for (a, b) in node.arms.iter().zip(sorted.iter()) {     
                if a.pat != b.pat {
                    let a_str = get_pat_path(a).unwrap();
                    let b_str = get_pat_path(b).unwrap();
                    let str = format!("{} should sort before {}", b_str, a_str);
                    self.err = Some(syn::Error::new_spanned(&b.pat, str));
                    return ;
                }
            }
        }

    
}

/// 把Path类型转换为字符串 AA::BB::CC::DD
#[allow(dead_code)]
fn get_path_string(p: &syn::Path) -> String {
    p.segments.iter().map(|s| {
        s.ident.to_string()
    }).collect::<Vec<_>>().join("::")
    
}

/// 取出枚举类型Arm.pat 的Path
#[allow(dead_code)]
fn get_pat_path(arm: &syn::Arm) -> syn::Result<String> {
    let r = match  &arm.pat {
        // syn::Pat::Box(_) => todo!(),
        syn::Pat::Ident(v) => v.ident.to_string(),
        // syn::Pat::Lit(_) => todo!(),
        // syn::Pat::Macro(_) => todo!(),
        // syn::Pat::Or(_) => todo!(),
        syn::Pat::Path(v) => get_path_string(&v.path),
        // syn::Pat::Range(_) => todo!(),
        // syn::Pat::Reference(_) => todo!(),
        // syn::Pat::Rest(_) => todo!(),
        // syn::Pat::Slice(_) => todo!(),
        syn::Pat::Struct(v) => get_path_string(&v.path),
        // syn::Pat::Tuple(_) => todo!(),
        syn::Pat::TupleStruct(v) => get_path_string(&v.path),
        // syn::Pat::Type(_) => todo!(),
        // syn::Pat::Verbatim(_) => todo!(),
        syn::Pat::Wild(_) => "_".to_string(),
        _ => return Err(syn::Error::new_spanned(&arm.pat, "unsupported by #[sorted]"))
    };
    Ok(r)
    
}