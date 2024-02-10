use lexico;

macro_rules! ex {
  ($t:ident,$v:expr) => {
    println!("{} ({}) -> {:?}", $v, stringify!($t), lexico::$t($v));
  }
}

fn main() {
  ex!(i128, 56544);
  ex!(u128, 56544);

  ex!(i8, 3);
  ex!(i8, -128);
  ex!(u8, 255);

  ex!(i16, -2);
  ex!(i16,  2);
  ex!(u16, 65535);
  ex!(i16, -32768);

  ex!(i32, 5);
  ex!(u32, 5);
  ex!(i32, -10);

  ex!(i64, -1);
  ex!(i64, 100);
  ex!(u64, 4_000_000_000_000_000);

  ex!(f32, 4.2);
  ex!(f32, 0.42);
  ex!(f32, -10.0);
  ex!(f32, 2.1e6);
  ex!(f32, 2.1e20);
  ex!(f32, -2.1e6);

  ex!(f64, -2.1e6);
  ex!(f64, 2.1e2);
  ex!(f64, 2.1000000001e2);
}
