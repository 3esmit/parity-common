// Copyright 2026 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]
#![deny(deprecated)]

use uint::construct_uint;

construct_uint! {
	pub struct U64(1);
}
construct_uint! {
	pub struct U256(4);
}
construct_uint! {
	pub struct U512(8);
}

#[test]
fn maximum_values_preserve_tuple_construction() {
	const MAX: U256 = U256::MAX;
	assert_eq!(MAX, U256([u64::MAX; 4]));
	assert_eq!(U64::MAX, U64([u64::MAX]));
	assert_eq!(U512::MAX, U512([u64::MAX; 8]));
}

#[test]
fn primitive_conversion_boundaries() {
	macro_rules! check {
		($($ty:ty),+ $(,)?) => {$(
			let max = U256::from(<$ty>::MAX as u128);
			assert_eq!(<$ty>::try_from(max), Ok(<$ty>::MAX));
			assert_eq!(<$ty>::try_from(max + U256::one()),
				Err(concat!("integer overflow when casting to ", stringify!($ty))));
		)+};
	}
	check!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128);
	assert_eq!(U256::from(u32::MAX).as_u32(), u32::MAX);
	assert_eq!(U256::from(u64::MAX).as_u64(), u64::MAX);
	assert_eq!(U256::from(usize::MAX).as_usize(), usize::MAX);
}

#[test]
fn carry_borrow_and_multiplication_boundaries() {
	assert_eq!(U256::MAX.overflowing_add(U256::one()), (U256::zero(), true));
	assert_eq!(U256::zero().overflowing_sub(U256::one()), (U256::MAX, true));
	assert_eq!(U256::MAX.overflowing_mul(U256::from(2)), (U256::MAX - U256::one(), true));
	assert_eq!(U512::MAX.overflowing_mul(U512::one()), (U512::MAX, false));
}

#[test]
fn endian_roundtrip_preserves_words() {
	let value = U256([u64::MAX, u32::MAX as u64, 0, 1]);
	assert_eq!(U256::from_big_endian(&value.to_big_endian()), value);
	assert_eq!(U256::from_little_endian(&value.to_little_endian()), value);
}
