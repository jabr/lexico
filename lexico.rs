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
