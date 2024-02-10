# lexico

Lexicographically orderable number encodings based on [Sainath Mallidi's notes on "Storing data in order"](https://cornerwings.github.io/2019/10/lexical-sorting/).

## Example

```rust

  lexico::i128(56544)
  // => [128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 224]
  lexico::u128(56544);
  // => [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 224]

  lexico::i8(3); // => [131]
  lexico::i8(-128); // => [0]
  lexico::u8(255); // => [255]

  lexico::i16(-2); // => [127, 254]
  lexico::i16(2); // => [128, 2]
  lexico::u16(65535); // => [255, 255]
  lexico::i16(-32768); // => [0, 0]

  lexico::i32(5); // => [128, 0, 0, 5]
  lexico::u32(5); // => [0, 0, 0, 5]
  lexico::i32(-10); // => [127, 255, 255, 246]

  lexico::i64(-1);
  // => [127, 255, 255, 255, 255, 255, 255, 255]
  lexico::i64(100);
  // => [128, 0, 0, 0, 0, 0, 0, 100]
  lexico::i64(4_000_000_000_000_000);
  // => [0, 14, 53, 250, 147, 26, 0, 0]

  lexico::f32(4.2); // => [192, 134, 102, 102]
  lexico::f32(0.42); // => [190, 215, 10, 61]
  lexico::f32(-10.0); // => [62, 224, 0, 0]
  lexico::f32(2.1e6); // => [202, 0, 44, 128]
  lexico::f32(2.1e20); // => [225, 54, 37, 94]
  lexico::f32(-2.1e6); // => [53, 255, 211, 128]

  lexico::f64(-2.1e6);
  // => [62, 191, 250, 112, 0, 0, 0, 0]
  lexico::f64(2.1e2);
  // => [192, 106, 64, 0, 0, 0, 0, 0]
  lexico::f64(2.1000000001e2);
  // => [192, 106, 64, 0, 0, 5, 94, 100]
```

## License

This project is licensed under the terms of the [MIT license](LICENSE.txt).
