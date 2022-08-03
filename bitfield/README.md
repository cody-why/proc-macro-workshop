#
* 这个项目目的是设计一个结构属性可以用1位到64位的数据
我们设计一个宏#[bitfield],标记 B1 到 B64 的位宽
 
* 创建一个名为 bitfield::Specifier 的特征，并带有一个相关的常量 BITS，
并编写一个类似函数的过程宏来定义一些类型 B1 到 B64 具有相应的说明符特征的实现
* The B* types can be anything,只需要在struct上面加上#[bitfield]
* 类似 pub enum B1 {}
* 这个项目有2个crate,一个impl用于过程宏，另一个用于Specifier trait 和 B 类型的普通库,也从程序宏包中重新导出,以便使用

1. 
```rust
// 实现类似这样的trait
pub trait Specifier {
    const BITS: usize;
}
enum B1 {}
impl Specifier for B1 {
    const BITS: usize = 1;
}
```

2. 
* 编写一个属性宏，用一个字节数组表示正确的数据大小
* 通过Specifier::BITS 求和来计算
```rust
// 加入一个属性和一个字段
// data: [u8; 4],
#[repr(C)]
pub struct MyFourBytes {
    data: [u8; #size],
}

```

3. 
* 编写getters 和 setters
4. 

5. 
* 根据bits指定getter 和 setter 的参数类型 u8,u16,u32,u64
* const BITS:u8 = #bits; // 占用的位数
* type UNIT; // 关联的储存类型
type InOut; // 输入输出参数类型

6. 
* #[derive(BitfieldSpecifier)]这个宏指示枚举成员用的位数,实现了Specifier trait,就像B1..B64,BITS是成员用的最大的位数,例如成员是0b001,则BITS=3,表示这个枚举要用3位数据内存.
* 实现bool的Specifier trait
* 增加把enum转数值的trait: try_from(), to_bytes()
* 输入输出的枚举类型  type InOut = #name;
7. 
* enum 不要假设编译器使用任何特定的方案，如 PREV+1
* 不要担心如果判别式超出0..2^BITS范围会发生什么,我们将在稍后的测试用例中进行编译时检查，以确保它们在范围内

8. 
* then enums with non-power-of-two variants should fail to compile
* 枚举变量数量不是2的平方不能编译

9. 
* 判断枚举范围不是 0..2^BITS 无法编译
* 因为enum DeliveryMode 有8个成员,计算得到8位bit,如果第一个成员值从1开始,8位无法放得下,所以不符合CheckDiscriminantInRange特征


