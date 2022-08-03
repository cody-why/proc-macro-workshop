/**
 * @Author: plucky
 * @Date: 2022-07-30 15:59:18
 * @LastEditTime: 2022-08-03 21:22:27
 * @Description: 
 */

#[cfg(test)]
mod tests {
   use bits::BitOpt;
enum B4 {}

trait Specifier {
    const BITS: usize;
    type UNIT;
    type InOut;
    fn to_bytes(input: Self::InOut) -> Self::UNIT;
}
    
impl Specifier for B4 {
    const BITS: usize = 4;
    type UNIT=u8;
    type InOut=u8;

    fn to_bytes(input: Self::InOut) -> Self::UNIT {
        input
    }

}

pub struct MyFourBytes {
    data: [u8; 4],
    // a: B1,
    // b: B3,
    // c: B4,
    // d: B24,
}

impl MyFourBytes {
    pub fn new() -> Self {
        Self {
            data: [0; 4],
        }
    }

    pub fn get_c(&self) -> u64 {
        let start = 1;
        let mut bits = 24;
        let mut index = start / 8;
        let mut offset = start % 8;
        // let end = start + bits;
        let mut value = false as u64;
        let mut last_copy = 0;
        //假设数据是8位,开始4+8=12,第一次data[0]的4..8,取的低4位,第二次是data[1]的0..4,取的高4位
        while bits > 0 {
            // 这次能够copy的最小位数
            let copy_bits = std::cmp::min(bits, 8-offset);
            let end = offset + copy_bits;
            let b = self.data[index].get_bits(offset..end);
            value |= (b as u64) << last_copy;
            bits -= copy_bits;
            offset += copy_bits;
            if offset >= 8 {
                offset = 0;
                index += 1;
            }
            last_copy += copy_bits;
        }

        value
        
    }


    pub fn set_c(&mut self, v: u64) {
        let start = 1; // 存储位置
        let mut bits = 24; // 位数

        let mut index = start / 8; // 数组索引
        let mut offset = start % 8; // 位索引
        // let end = start + bits; // 结束位置
        
        let mut value = v; // 待copy的值
        
        //假设数据是8位,4+8=12,第一次data[0]的4..8,第二次是data[1]的0..4
        while bits > 0 {
                // 这次能够copy的位数
            let copy_bits = std::cmp::min(bits, 8-offset);
            let end = offset + copy_bits;
            let b = value as u8;
            self.data[index].set_bits(offset..end, b);
            value >>= copy_bits;
            bits -= copy_bits;
            offset += copy_bits;
            if offset >= 8 {
                offset = 0;
                index += 1;
            }
        }
        
    }
}



    #[test]
    fn test_name() {
        println!("=={},{}",(1-1)/8+1,8%8);
        let mut bitfield = MyFourBytes::new();
        let v = 111150u64;
        println!("{:08b}", v);
        bitfield.set_c(v);
        println!("============");
        println!("{:08b}", bitfield.get_c());
        assert_eq!(v, bitfield.get_c());

        // let a = true;
        // let b = a as u8 ==1;
        // let c = b as u8;
       
        //  println!("{}", c);
        // 判断b是不是bool类型
       
       

        
    }
}
