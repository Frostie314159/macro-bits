macro_rules! generate_field_size {
    ($field_type:ty, $field_size:expr) => {
        $field_size
    };
    ($field_type:ty) => {
        core::mem::size_of::<$field_type>()
    };
}
macro_rules! impl_fixed_rw {
    (
        $(#[$struct_attr:meta])*
        $struct_vis:vis struct $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident: $field_type:ty $(| $field_size:expr)?,
            )*
        }
    ) => {
        $struct_vis struct $struct_name {
            $(
                $(#[$field_attr])*
                $field_vis $field_name: $field_type,
            )*
        }
        impl $struct_name {
            #[doc(hidden)]
            const __SIZE_INTERNAL: usize = $(generate_field_size!($field_type $(, $field_size)?) + )* 0;
        }
    };
}
impl_fixed_rw! {
    pub struct Test {
        pub x: u8 | 4,
    }
}
