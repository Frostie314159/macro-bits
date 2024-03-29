#![no_std]

#[macro_use]
mod bitfield;
#[macro_use]
mod bit;
#[macro_use]
mod serializable_enum;
#[macro_use]
mod incomplete_const_array;
/// Utility macros for shortend syntaxes for definitions.
mod gen_def;

pub use defile;

#[macro_export]
macro_rules! generate_conversions {
    (u8, $representation:ty, $target_type:ty) => {
        ::macro_bits::generate_conversions!(u8, $representation, $target_type, 0);
        ::macro_bits::generate_conversions!(u16, $representation, $target_type);
    };
    (u16, $representation:ty, $target_type:ty) => {
        ::macro_bits::generate_conversions!(u16, $representation, $target_type, 0);
        ::macro_bits::generate_conversions!(u32, $representation, $target_type);
    };
    (u32, $representation:ty, $target_type:ty) => {
        ::macro_bits::generate_conversions!(u32, $representation, $target_type, 0);
        ::macro_bits::generate_conversions!(u64, $representation, $target_type);
    };
    (u64, $representation:ty, $target_type:ty) => {
        ::macro_bits::generate_conversions!(u64, $representation, $target_type, 0);
        ::macro_bits::generate_conversions!(u128, $representation, $target_type, 0);
    };
    ($to:ty, $representation:ty, $target_type:ty, $_:expr) => {
        impl From<$to> for $target_type {
            fn from(value: $to) -> Self {
                Self::from_bits(value as $representation)
            }
        }
        impl From<$target_type> for $to {
            fn from(value: $target_type) -> Self {
                value.into_bits() as $to
            }
        }
    };
}
