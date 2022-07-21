# 实现步骤

1.实现解析struct

2.实现Debug trait

```rust
impl fmt::Debug for #name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .finish()
    }
}
```

3.实现属性解析和指定格式化

```rust
 .field(#name_str, &format_args!(#fmt, self.#name))
```

4.对T泛型参数的增加Debug限定

```rust
impl <T:Debug> fmt::Debug for #name <T> {
}
```

5.PhantomData是用于标记假装他拥有T泛型,因为他的值是零大小类型,不需要Debug,要求给它开后门,不用给它加Debug:

```rust
pub struct Field<T,U> {
    marker: PhantomData<T>,
    other: U,
}
```

PhantomData的T不需要加限定

6.除非其他属性用到T,例如:other: T,那么T就要加限定了通

7.错误提示:Id没有实现Debug.

例如values: Vec<T::Value>,是关联类型,则需要在where子句中加T::Value的Debug限定,不能在T加限定

```rust
impl<T: Trait> Debug for Field `<T>`  where T::Value: Debug,
{...}
```

例如values: Vec<T::Value>,T::Value就是T的关联类型,将关联类型T::Value标识为 syn::TypePath

怎么找出TypePath:

1.先给Field手动加上where T::Value: Debug,打印出语法树,分析树结构

2.T::Value在树中表现为:path的segments有2个segment,首个segment是T,第2个segment是Value,找出这种类型的TypePath.

3.syn: visit,启用features = ["visit"]

功能是遍历返回你希望得到的类型,例如TypePath:

```rust
fn visit_type_path(&mut self, node: &'ast syn::TypePath) {...}
```

8.错误提示:Id没有实现'Debug',这一关主要是实现属性指定#[debug(bound = "T::Value: Debug")],直接给where加上bound的值

惯例,屏蔽assertas,println!("attrs={:#?}", input.attrs),分析语法树.

```rust
// 8 取得bound = "T::Value: Debug"
let bound = get_struct_attr(&self.attrs, "bound");
if let Some(v) = bound {
    // 第8关,如果属性指定限定类型,只需要在where子句中加上限定类型即可   
    generics.make_where_clause();
    let clause = syn::parse_str(v.as_str()).unwrap();
    generics.where_clause.as_mut().unwrap().predicates.push(clause);
    return generics;
}
```
