/**
 * @Author: plucky
 * @Date: 2022-08-02 23:48:32
 * @LastEditTime: 2022-08-10 16:28:36
 * @Description:
 */

pub trait SizeType {
    type CheckType;
}

pub struct TotalSize<T>(::std::marker::PhantomData<T>);

pub trait TotalSizeIsMultipleOfEightBits {}

macro_rules! impl_total_size_for {
  ($(($n:expr,$name:ident)),*) => {
      $(
          pub enum $name {}
          impl SizeType for TotalSize<[();$n]>{
              type CheckType = $name;
          }
      )*
  };
}

impl_total_size_for!(
    (0, ZeroMod8),
    (1, OneMod8),
    (2, TwoMod8),
    (3, ThreeMod8),
    (4, FourMod8),
    (5, FiveMod8),
    (6, SixMod8),
    (7, SevenMod8)
);

impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {}
pub trait CheckTotalSizeMultipleOf8
where
    <Self::Size as SizeType>::CheckType: TotalSizeIsMultipleOfEightBits,
{
    type Size: SizeType;
}

pub trait DiscriminantInRange {}

pub enum True {}
pub enum False {}

pub trait DispatchTrueFalse {
    type Out;
}

impl DiscriminantInRange for True {}

impl DispatchTrueFalse for [(); 0] {
    type Out = False;
}

impl DispatchTrueFalse for [(); 1] {
    type Out = True;
}

pub trait CheckDiscriminantInRange<A>
where
    <Self::CheckType as DispatchTrueFalse>::Out: DiscriminantInRange,
{
    type CheckType: DispatchTrueFalse;
}



#[cfg(test)]
mod testcheck{
    use crate::check::CheckDiscriminantInRange;
    const BITS: usize =1;

#[allow(dead_code)]
enum Abc {
    A,
    B,
    C,
}

impl CheckDiscriminantInRange<[(); Abc::A as usize]> for Abc{
    // [();1]
    type CheckType = [(); ((Abc::A as usize) < (0x01_usize << BITS)) as usize];
}
// impl CheckDiscriminantInRange<[(); Abc::B as usize]> for Abc{
//     // [();1]
//     type CheckType = [(); ((Abc::B as usize) < (0x01_usize << BITS)) as usize];
// }
// 因为C的值是2,需要2位bit,所以2<2是false,数字0,CheckType=[(),0]的值是False,False没有实现DiscriminantInRange,不符合CheckDiscriminantInRange
// impl crate::check::CheckDiscriminantInRange<[(); Abc::C as usize]> for Abc{
//     // [();0]
//     type CheckType = [(); ((Abc::C as usize) < (0x01_usize << BITS)) as usize];
// }

    #[test]
    fn testcheck(){
        println!("{},{}", (Abc::A as usize) , (0x01_usize << BITS));
    }
    
}