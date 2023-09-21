#[macro_export]
/// Generate a serializable enum.
/// 
/// An Unknown field is always generated, which contains a variable, which type is the enum representation.
/// ```
/// use macro_bits::serializable_enum;
/// 
/// serializable_enum! {
///     #[derive(Debug, PartialEq)]
///     pub enum ABC: u8 {
///         /// This is a doc comment too.
///         A => 0,
///         B => 1,
///         C => 2
///     }
/// }
/// assert_eq!(ABC::from_representation(2), ABC::C);
/// assert_eq!(ABC::from_representation(3), ABC::Unknown(3));
/// assert_eq!((ABC::C).to_representation(), 2);
/// ```
macro_rules! serializable_enum {
    (
        $(#[$enum_attr:meta])*
        $enum_visibility:vis enum $enum_name:ident : $representation:ty {
            $(
                $(#[$field_attr:meta])*
                $field_name:ident => $field_value:expr
            ),*
        }
    ) => {
        ::defile::defile!{
            $(#[$enum_attr])*
            $enum_visibility enum $enum_name {
                $(
                    $(#[$field_attr])*
                    $field_name,
                )*
                Unknown($representation)
            }
            impl $enum_name {
                pub fn from_representation(value: $representation) -> Self {
                    use $enum_name::*;
                    match value {
                        $(
                            $field_value => $field_name,
                        )*
                        x => Self::Unknown(x)
                    }
                }
                pub fn to_representation(self) -> $representation {
                    use $enum_name::*;
                    match self {
                        $(
                            $field_name => $field_value,
                        )*
                        Self::Unknown(x) => x
                    }
                }
            }
            ::macro_bits::generate_conversions!($representation, $representation, $enum_name);
        }
    };
}