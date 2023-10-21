#[macro_export]
/// Generate consts
/// 
/// ```
/// use macro_bits::gen_consts;
/// 
/// gen_consts! {
///     const_type: u8,
///     A => 0x01,
///     B => 0x02
/// }
/// assert_eq!(A, 0x01);
/// ```
macro_rules! gen_consts {
    (
        const_type: $const_type:ty,
        $(
            $const_ident:ident => $const_value:expr
        ),*
    ) => {
        $(
            pub const $const_ident: $const_type = $const_value;
        )*
    };
}