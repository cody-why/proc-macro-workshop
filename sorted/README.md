# 属性宏 #[sorted]

###### 属性宏中,如果只返回空TokenStream,等于删掉了用户原始代码

1. 第1关
提示说: syn要打开 "full" feature. 用syn::Item解析enum

```rust
#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    // 第一个参数代表宏本身#[xxx]，第二个参数代表被修饰的代码块
    let _ = parse_macro_input!(args as syn::AttributeArgs);
    let item= parse_macro_input!(input as syn::Item);
}
```

2. 第2关
用err.to_compile_error().into()指示错误位置,#[sorted]只能匹配enum

3. 第3关
我们需要检查枚举的变量是否按字典顺序出现！
将错误指向顺序错误的变量

4. 第4关
错误提示有其他信息:warning: unused import: `std::env::VarError`
因为返回错误时,没有返回用户原始代码,在属性宏中,原始代码一定要返回的,如果是返回空TokenStream,等于删掉了用户原始代码
所以无论什么时候都应该把原始代码也包含返回