#[macro_export]
/// Generate a mask from one or more bit indices.
///
/// ```
/// use macro_bits::bit;
///
/// const SINGLE_BIT: u8 = bit!(0);
/// const MULTIPLE_BITS: u8 = bit!(1,2);
///
/// assert_eq!(SINGLE_BIT, 1);
/// assert_eq!(MULTIPLE_BITS, 6);
/// ```
macro_rules! bit {
    ($index:expr) => {
        (1 << $index)
    };
    ($($index:expr), +) => {
        $(bit!($index) | )+ 0
    };
}
#[macro_export]
/// Check if a bit is set.
///
/// ```
/// use macro_bits::{check_bit, bit};
///
/// const MASK: u8 = bit!(1,2);
///
/// assert!(check_bit!(0xff, MASK));
/// ```
macro_rules! check_bit {
    ($flags:expr, $mask:expr) => {
        ($flags & $mask != 0x0)
    };
}
#[macro_export]
/// Set a bit.
///
/// ```
/// use macro_bits::{set_bit, bit};
///
/// const MASK1: u8 = bit!(0,1);
/// const MASK2: u8 = bit!(2,3);
///
/// let mut data = 0;
///
/// set_bit!(data, MASK1);
///
/// let condition = true;
/// set_bit!(data, MASK2, condition);
///
/// assert!(data == MASK1 | MASK2);
/// ```
macro_rules! set_bit {
    ($flags:expr, $mask:expr) => {
        $flags |= $mask
    };
    ($flags:expr, $mask:expr, $value:expr) => {
        if $value {
            set_bit!($flags, $mask);
        }
    };
}
