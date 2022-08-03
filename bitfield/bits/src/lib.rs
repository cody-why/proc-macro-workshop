/**
 * @Author: plucky
 * @Date: 2022-07-30 16:07:18
 * @LastEditTime: 2022-08-03 21:49:03
 * @Description: 
 */

pub mod bits;
pub use bits::BitOpt;

bit_opt_impl! {u8 u16 u32 u64 usize i8 i16 i32 i64 isize}