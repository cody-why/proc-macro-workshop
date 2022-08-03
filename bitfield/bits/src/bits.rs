/**
 * @Author: plucky
 * @Date: 2022-07-29 21:40:02
 * @LastEditTime: 2022-08-03 21:51:34
 * @Description: 
 */

use std::ops::Range;

 pub trait BitOpt {
    /// 位长度
    /// # Examples
    /// ```
    /// u8::length(); // 8
    /// ```
    fn length() -> usize;
    /// 获取位的值
    /// # Examples
    /// ```
    /// 0b1010_1010u8.get_bit(0); // false
    /// ```
    fn get_bit(&self, bit: usize) -> bool;
    /// 获取指定范围的位的值
    /// # Examples
    /// ```
    /// 0b1010_1010u8.get_bits(0..2); // 0b10
    /// ```
    fn get_bits(&self, range: Range<usize>) -> Self;
    /// 设置位的值
    /// # Examples
    /// ```
    /// let mut b = 0b1010_1010u8;
    /// b.set_bit(0, true); // 0b1010_1011
    /// ```
    fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self;
    /// 把value设置到指定的位置上
    /// # Examples
    /// ```
    /// let mut b = 0b1010_1010u8;
    /// b.set_bits(0..2, 0b11); // 0b1010_1011
    /// ```
    fn set_bits(&mut self, range: Range<usize>, value: Self) -> &mut Self;
}

#[macro_export]
macro_rules! bit_opt_impl {
    ( $($t:ty)* ) => ($(
        impl BitOpt for $t {
            fn length() -> usize {
                ::core::mem::size_of::<Self>() as usize * 8
            }

            fn get_bit(&self, bit: usize) -> bool {
                assert!(bit < Self::length());
                (*self & (1 << bit)) != 0
            }

            fn get_bits(&self, range: std::ops::Range<usize>) -> Self {
                assert!(range.start < Self::length());
                assert!(range.end <= Self::length());
                assert!(range.end > range.start);

                let shift_bits = Self::length() - range.end;
                let bits = *self << shift_bits >> shift_bits;

                bits >> range.start
            }

            fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
                assert!(bit < Self::length());
                let mask = 1 << bit;
                if value {
                    *self |= mask;
                } else {
                    *self &= !(mask);
                }
                self
            }


            fn set_bits(&mut self, range: std::ops::Range<usize>, value: Self) -> &mut Self {
                assert!(range.start < Self::length());
                assert!(range.end <= Self::length());
                assert!(range.end > range.start);

                let shift_bits = Self::length() - range.end;
                let mask = !0 << shift_bits >> shift_bits >> range.start << range.start;
                *self = (*self & !mask) | (value << range.start);

                self
            }
        }
    )*)
}


// bit_opt_impl! {u8 u16 u32 u64 usize i8 i16 i32 i64 isize}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_opt() {
        assert_eq!(8, u8::length());
        let mut field = 0b11110010u8;
        field.set_bit(0, true);
        field.set_bit(0, 1u8 == 1);
        assert_eq!(0b11110011, field);
        println!("v1={:08b}", field);
        field.set_bits(1..5, 0b00000111u8);
        println!("v2={:08b}", field);
        assert_eq!(0b11101111, field);
        
    }
}

