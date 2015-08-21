// As the name implies, unsigned integers (u8, u16, u32, u64) cannot be negative
#[test]
fn unsigned_ints() {
  assert!(u8::min_value() == __);
}

// Unsigned integers can be reduced only as far as their minimum value of 0
#[test]
fn sub_unsigned_int() {
  let mut num:u8 = 10;
  num -= 10;
  assert!(num __ u8::min_value());
}

// Signed integers(i8, i16, i32, i64), on the other hand, can be negative
#[test]
fn signed_ints() {
  assert!(i8::min_value() __ 0);
}

// Signed integers can be reduced below zero, as far as their minimum value
#[test]
fn sub_signed_int() {
  let mut num:i8 = 0;
  let neg:i8 = __;
  num += neg;
  assert!(num == i8::min_value());
}

// Addition of positive integers works much the same for signed and unsigned numbers
fn add_numbers() {
  let mut sig:i8 = 0;
  let mut unsig:u8 = 0;
  sig += __;
  unsig += __;
  assert!(sig == i8::max_value() && unsig == u8::max_value());
}
