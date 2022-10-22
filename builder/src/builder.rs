use std::{iter::Map, slice::Iter};

use darling::{FromField};
use proc_macro2::{TokenStream, Ident};
use quote::format_ident;
use syn::{ DeriveInput, Type};

type TokenStreamIter<'a> = Map<Iter<'a, Field>, fn(&'a Field) -> TokenStream>;

// 最后一关,用绝对路径
type Option<T> = std::option::Option<T>;


/// 属性 #[builder(each = "arg")]
#[derive(Debug,Default,FromField)]
#[darling(attributes(builder),default)]
struct Opts {
    each: Option<String>,
    default: Option<String>,
}


/// Punctuated<Field, Token![,]> 中的Field和属性宏
struct Field {
    name: Ident,
    ty: Type,
    attr: Opts,
}

// lib.rs只写宏定义,builder.rs写宏实现
pub struct BuilderContext {
    name: Ident,
    // fields: Punctuated<Field, Token![,]>,
    fields: Vec<Field>,
}


impl BuilderContext {
    /// Create a new builder context with DeriveInput.
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;

         // input.data枚举是Struct(DataStruct)中的fields枚举是Named(FieldsNamed)中的named
        let fields = match input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed {named, .. }),
                ..
            }) => named,
            _ => panic!("Only structs are supported"),
        };
        
        let fields = fields.into_iter().map(|f|{
            // println!("{:#?}", f);
            let attr = Opts::from_field(&f).unwrap_or_else(|op|{
                 panic!("{:?}", op)
            });
            
            println!("{:?}", attr);
            // let attr = Opts::default();
            Field {
                name: f.ident.unwrap(),
                ty: f.ty,
                attr,
            }
        }).collect();

        Self { name, fields }

    }

    /// Generate the builder implementation.
    pub fn generate(&self) -> TokenStream {
        // 输入的结构:Command
        let name = &self.name;
        // 辅助结构:CommandBuilder,用于生成Command
        // let builder_name = Ident::new(&format!("{}Builder", name), name.span());
        let builder_name = format_ident!("{}Builder", name);

        // 属性: executable: Option<String>,
        let fields = self.gen_fields();
        // methods: fn executable(mut self, v: impl Into<String>) -> Self{self.executable = Some(v); self}
        let methods = self.gen_methods();

        // 用builder属性分配#name结构,Command{executable:"helLo"}
        let assigns = self.gen_assigns();

        // Command::builder().executable("helLo").args(vec![]).envs(vec![]).build()

        quote::quote! {
            // 用输入的结构的属性创建builder结构
            // CommandBuilder{executable: Option<String>, args: Vec<String>, env: Vec<String>, current_dir: Option<String>}
            #[derive(Debug,Default)]
            pub struct #builder_name {
                #(#fields,)*
            }
            // 生成方法,没有逗号分割
            impl #builder_name {
                #(#methods)*
            }

            impl #builder_name {
                pub fn build(&mut self) -> std::result::Result<#name, &'static str> {
                    // 创建Command结构
                    Ok(#name{
                        #(#assigns,)*
                    })
                }

            }

            // Command 的 builder 实现
            impl #name {
                pub fn builder() ->#builder_name{
                     Default::default()
                }
            }
        }
    }

    /// 返回一个iter, struct的属性,name:type
    fn gen_fields(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let name = &f.name;
            let ty = &f.ty;
            quote::quote! {
                #name:std::option::Option<#ty>
            }
        })
    }

    /// 返回一个iter, 生成属性的set方法, 某值(v){self.某值=v;}
    fn gen_methods(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            // let name = f.ident.as_ref().unwrap();
            let name = &f.name;
            let ty = &f.ty;
     
            // 这里需要&str,not &String
            if let Some(each) = f.attr.each.as_deref(){
                let (isvec,ty) = get_vec_type(ty);
                if isvec {
                    let each = Ident::new(each, name.span());
                    return  quote::quote! {
                        pub fn #each(&mut self, v:impl Into<#ty>) ->&mut Self {
                            let mut vec = self.#name.take().unwrap_or_default();
                            vec.push(v.into());
                            self.#name = Some(vec);
                            self
                        }
                    }
                }
            }
            
            //fn executable(mut self, v: impl Into<String>) -> Self {self.executable = Some(v); self}
            quote::quote! {
               pub fn #name(&mut self, v: impl Into<#ty>) -> &mut Self {
                    self.#name = Some(v.into());
                    self
                }
            }
        })
    }

    /// 返回一个iter, 用builder的属性分配给#name结构
    fn gen_assigns(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            // let name = f.ident.as_ref().unwrap();
            let name = &f.name;
            // unwrap_or_default 可以是None
            quote::quote! {
               #name: self.#name.take().unwrap_or_default()
            }
        })
    }
}


#[allow(dead_code)]
/// 取Option的值的类型,返回(是否Option,值的类型)
pub fn get_option_type(ty: &Type) -> (bool, &Type){
    get_inner_type(ty, "Option")
}

#[allow(dead_code)]
/// 取Vec的值的类型,返回(是否Vec,值的类型)
pub fn get_vec_type(ty: &Type) -> (bool, &Type){
    get_inner_type(ty, "Vec")
}


/// 取出inner的类型,比如:Option<String>,Vec<String>,取出String.
/// 返回(是否inner,inner的类型)
fn get_inner_type<'a>(ty: &'a Type, name:&str) -> (bool, &'a Type) {
    // 匹配 syn::Type::Path(ref path) {segments[0].ident="Option"}
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
