#[macro_export]
/// Allows the creation of a const array without specifying all elements.
/// 
/// ```
/// use macro_bits::incomplete_const_array;
/// 
/// incomplete_const_array! {
///     #[filler(0)] // Has to be first.
///     /// Just some array.
///     pub const ARRAY: [usize; 4] = [
///         1 => 1337,
///         3 => 42
///     ];
/// }
/// assert_eq!(ARRAY, [0, 1337, 0, 42]);
/// ```
macro_rules! incomplete_const_array {
    (
        #[filler($filler:expr)]
        $(#[$const_attr:meta])*
        $const_vis:vis const $const_name:ident: [$array_type:ty; $array_length:expr] = [
            $(
                $idx:expr => $value:expr
            ),*
        ];
    ) => {
        $(#[$const_attr])*
        $const_vis const $const_name: [$array_type; $array_length] = {
            let mut array = [$filler; $array_length];
            
            $(
                array[$idx] = $value;
            )*

            array
        };
    };
}