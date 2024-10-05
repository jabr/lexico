// Based on https://cornerwings.github.io/2019/10/lexical-sorting/

use std::mem;

macro_rules! lu {
  ($t:ident) => {
    #[inline(always)]
    #[no_mangle]
    pub fn $t(x: $t) -> [u8; mem::size_of::<$t>()] {
      return x.to_be_bytes();
    }
  }
}

lu!(u8);
lu!(u16);
lu!(u32);
lu!(u64);
lu!(u128);

macro_rules! li {
  ($t:ident) => {
    #[inline(always)]
    #[no_mangle]
    pub fn $t(x: $t) -> [u8; mem::size_of::<$t>()] {
      return (x ^ $t::MIN).to_be_bytes();
    }
  }
}

li!(i8);
li!(i16);
li!(i32);
li!(i64);
li!(i128);

macro_rules! lf {
  ($t:ident, $i:ident) => {
    #[inline]
    #[no_mangle]
    pub fn $t(x: $t) -> [u8; mem::size_of::<$i>()] {
      let mut res = unsafe { mem::transmute::<$t, $i>(x) };
      if res < 0 { res = $i::MIN - res; }
      return $i(res);
    }
  }
}

lf!(f32, i32);
lf!(f64, i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_lexically_ordered_byte_arrays() {
      assert_eq!(i128(56544), [ 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 224 ]);
      assert_eq!(u128(56544), [   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 224 ]);

      assert_eq!(i8(3), [ 131 ]);
      assert_eq!(i8(-128), [ 0 ]);
      assert_eq!(u8(255), [ 255 ]);

      assert_eq!(i16(-2), [ 127, 254 ]);
      assert_eq!(i16(2), [ 128, 2 ]);
      assert_eq!(u16(65535), [ 255, 255 ]);
      assert_eq!(i16(-32768), [ 0, 0 ]);

      assert_eq!(i32(5), [ 128, 0, 0, 5 ]);
      assert_eq!(u32(5), [ 0, 0, 0, 5 ]);
      assert_eq!(i32(-10), [ 127, 255, 255, 246 ]);

      assert_eq!(i64(-1), [ 127, 255, 255, 255, 255, 255, 255, 255 ]);
      assert_eq!(i64(100), [ 128, 0, 0, 0, 0, 0, 0, 100 ]);
      assert_eq!(u64(4_000_000_000_000_000u64), [ 0, 14, 53, 250, 147, 26, 0, 0 ]);

      assert_eq!(f32(4.2), [ 192, 134, 102, 102 ]);
      assert_eq!(f32(0.42), [ 190, 215, 10, 61 ]);
      assert_eq!(f32(-10.0), [ 62, 224, 0, 0 ]);
      assert_eq!(f32(2.1e6), [ 202, 0, 44, 128 ]);
      assert_eq!(f32(2.1e20), [ 225, 54, 37, 94 ]);
      assert_eq!(f32(-2.1e6), [ 53, 255, 211, 128 ]);

      assert_eq!(f64(-2.1e6), [ 62, 191, 250, 112, 0, 0, 0, 0 ]);
      assert_eq!(f64(2.1e2), [ 192, 106, 64, 0, 0, 0, 0, 0 ]);
      assert_eq!(f64(2.1000000001e2), [ 192, 106, 64, 0, 0, 5, 94, 100 ]);
    }
}
