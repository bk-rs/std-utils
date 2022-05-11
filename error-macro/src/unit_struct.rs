//
#[macro_export]
macro_rules! unit_struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident;
    ) => {
        $( #[$meta] )*
        #[derive(Debug)]
        $pub struct $name;

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{:?}", self)
            }
        }

        #[cfg(feature = "std")]
        impl ::std::error::Error for $name {}
    }
}
