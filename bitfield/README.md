#
* 我们设计一个宏#[bitfield],标记 B1 到 B64 的位宽
* 
* 创建一个名为 bitfield::Specifier 的特征，并带有一个相关的常量 BITS，
* 并编写一个类似函数的过程宏来定义一些类型 B1 到 B64 具有相应的说明符特征的实现
* The B* types can be anything,只需要在struct上面加上#[bitfield]
* 产生类似 pub enum B1 {}
* 这个项目有2个crate,一个impl用于过程宏，另一个用于Specifier trait 和 B 类型的普通库,也从程序宏包中重新导出,以便使用

1. 
```rust
// 实现类似这样的trait
pub trait Specifier {
    const BITS: u8;
}
enum B24 {}
impl Specifier for B24 {
    const BITS: u8 = 24;
}
```

2. 
