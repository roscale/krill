use core::ops::RangeInclusive;

#[inline]
pub fn page_of(address: u16) -> u8 {
    (address >> 8) as u8
}

pub trait BitOperations {
    fn get_bit(&self, index: u8) -> bool;
    fn set_bit(&mut self, index: u8, value: bool);
    fn get_bits(&self, range: RangeInclusive<u8>) -> Self;
    fn get_bits_unshifted(&self, range: RangeInclusive<u8>) -> Self;
    fn set_bits(&mut self, range: RangeInclusive<u8>, value: Self);
    fn set_bits_all(&mut self, range: RangeInclusive<u8>, value: bool);
}

macro_rules! impl_bit_operations_for {
    ($type: ty) => {
        impl BitOperations for $type {
            fn get_bit(&self, index: u8) -> bool {
                (self & (1 << index)) != 0
            }

            fn set_bit(&mut self, index: u8, value: bool) {
                let clear_bit = *self & !(1 << index);
                let set_bit = clear_bit | ((value as $type) << index);
                *self = set_bit;
            }

            fn get_bits(&self, range: RangeInclusive<u8>) -> Self {
                assert!(range.end() >= range.start());
                let mut mask = 0;
                mask.set_bits_all(range.clone(), true);
                (self & mask) >> range.start()
            }

            fn get_bits_unshifted(&self, range: RangeInclusive<u8>) -> Self {
                assert!(range.end() >= range.start());
                let mut mask = 0;
                mask.set_bits_all(range.clone(), true);
                self & mask
            }

            fn set_bits(&mut self, range: RangeInclusive<u8>, value: $type) {
                assert!(range.end() >= range.start());
                for i in range.clone() {
                    self.set_bit(i, value.get_bit(i - range.start()));
                }
            }

            fn set_bits_all(&mut self, range: RangeInclusive<u8>, value: bool) {
                assert!(range.end() >= range.start());
                for i in range {
                    self.set_bit(i, value);
                }
            }
        }
    };
}

impl_bit_operations_for!(u8);
impl_bit_operations_for!(u16);
impl_bit_operations_for!(u32);
impl_bit_operations_for!(u64);
impl_bit_operations_for!(usize);

#[allow(non_snake_case)]
pub trait Units {
    fn KiB(&self) -> Self;
    fn MiB(&self) -> Self;
    fn GiB(&self) -> Self;
}

macro_rules! impl_units_for {
    ($type: ty) => {
        impl Units for $type {
            fn KiB(&self) -> Self {
                *self * 1024
            }
            fn MiB(&self) -> Self {
                self.KiB() * 1024
            }
            fn GiB(&self) -> Self {
                self.MiB() * 1024
            }
        }
    };
}

impl_units_for!(u16);
impl_units_for!(u32);
impl_units_for!(u64);
impl_units_for!(usize);
impl_units_for!(i16);
impl_units_for!(i32);
impl_units_for!(i64);
impl_units_for!(isize);