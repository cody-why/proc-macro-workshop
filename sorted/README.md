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
因为返回错误时,没有返回用户原始代码,在属性宏中,如果返回空TokenStream,等于删掉了用户原始代码
所以无论什么时候都应该把原始代码也包含返回

5. 第5关
总的来说，通过这个测试的步骤是：
- 引入一个名为 `check` 的新程序属性宏#[sorted::check]。
- 将输入解析为 syn::ItemFn。
- 遍历函数体寻找匹配表达式。这部分可以使用 Syn 的 VisitMut 特征并编写访问者，
则最简单使用 visit_expr_match_mut 方法。

- 对于每个匹配表达式，确定它是否有 #[sorted]。如果是，请检查match语句块是否已排序并删除
属性列表中的 #[sorted] 属性。

* 打开提示网址See the module documentation for details.有example
* Syn的VisitMut 需要feature "visit-mut"
* 因为math上不能用#[sorted]修饰,想要输出而不出错,只好伪造返回代码了,输出再分析

```rust
r#"fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 1)
    }"#.parse().unwrap()
```
6. 第6关
这一关主要是Pat要匹配以下类型:
Pat::Path, Pat::TupleStruct, Pat::Struct.
关键代码:
get_path_string,get_pat_path

7. 第7关
需要提示[]类型不支持

8. 第8关
调试输出unsupported by #[sorted],看来有新类型
调试看看是什么类型:
pat: Ident 排序名字取ident: Ident
pat: Wild 排序名字是下划线_
