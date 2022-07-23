# 派生宏Seq!

###### 习惯的调试方法,返回 "".parse().unwrap(),不实现任何代码,所以不会报错,能输出想要的东西

1. 第1关
从https://github.com/dtolnay/syn/tree/master/examples/lazy-static 学会parse解析一个派生宏

N in 0..8 在语法树中分别是
syn::Ident, Token![in], syn::LitInt,Token![..], syn::LitInt.

2. 第2关
解析大括号里的内容为TokenStream
```rust
{
  expand_to_nothing!(N);
}
```

seq 宏不关心大括号内是写一个语句，还是一个函数，还是一个结构，或其他任何东西。
所以我们将{}内的内容作为TokenStream。


3. 第3关
构造生成的代码！生成重复输出的 TokenStream
循环次数由范围0..4指定,用循环计数替换指定的标识符:`N`

将扩展TokenStream，产生4行代码：
compile_error!(concat!("error number ", stringify!(0)));
compile_error!(concat!("error number ", stringify!(1)));
compile_error!(concat!("error number ", stringify!(2)));
compile_error!(concat!("error number ", stringify!(3)));

错误提示: error number 0 到 error: error number 3
可能是版本的问题,提示的信息稍有出入,复制当前提示的错误信息到03-expand-four-errors.stderr,测试通过

4. 第4关
将`fn f~N ()`扩展为：
fn f1() -> u64 { 1 *2 }
fn f2() -> u64 { 2 *2 }
fn f3() -> u64 { 3 *2 }

将Ident'f' Punct'~' Ident'N' 替换为 Ident'f1'
思路:判断i+1是不是'~',i+2是不是N,i+3是不是'~'

可选的,将`fn f~N~_suffix ()`扩展为
fn f1_suffix()

5. 第5关
```rust
// body
enum Interrupt {
  #(
      Irq~N,
  )*
}
// #(...)* 是循环表达式,展开为
enum Interrupt {
    Irq0,
    ...
    Irq15,
}
```
思路:
如果匹配到#(XXX)*,则循环展开XXX,否则展开整个body

匹配格式:
Punct# Group Punct*
判断Punct是不是'#',i+1是不是Group,i+2是不是'*'

6. 第6关
直接通过,不知道是不是因为第5关写得好

7. 第7关
range MIN..=MAX
这种范围就是在从MIN到MAX,包括MAX
前面几关的range MIN..MAX 产生的范围是不包括MAX的
思路:利用peek()查看如果有'=',那么end值加1

8. 第8关
提示的错误信息^指向出错的位置,这就是span的功劳
let _ = Missing~N;
        ^^^^^^^ not found in this scope

9. 第9关
他说不能直接用定义常量来指定seq!的上限,可以用macro_rules!来展开一个常量