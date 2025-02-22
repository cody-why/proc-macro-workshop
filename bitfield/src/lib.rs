// Crates that have the "proc-macro" crate type are only allowed to export
// procedural macros. So we cannot have one crate that defines procedural macros
// alongside other types of public APIs like traits and structs.
//
// For this project we are going to need a #[bitfield] macro but also a trait
// and some structs. We solve this by defining the trait and structs in this
// crate, defining the attribute macro in a separate bitfield-impl crate, and
// then re-exporting the macro from this crate so that users only have one crate
// that they need to import.
//
// From the perspective of a user of this crate, they get all the necessary APIs
// (macro, trait, struct) through the one bitfield crate.
pub use bitfield_impl::*;
pub use bits::BitOpt;
pub mod check;

mod tests;

pub trait Specifier {
    const BITS: usize; // 占用的位数
    type UNIT; // 关联的储存类型
    type InOut; // 关联的输入输出类型

    // 将输入值转换为对应存储的值
    fn to_bytes(input: Self::InOut) -> Self::UNIT;
    // 将存储的数值转换为输出值
    fn try_from(v: Self::UNIT) -> Self::InOut;
}