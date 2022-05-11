#[macro_export(local_inner_macros)]
macro_rules! internal_unit_struct {
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
    }
}

//
#[cfg(feature = "std")]
#[macro_export]
macro_rules! unit_struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident;
    ) => {
        $crate::internal_unit_struct! {
            $( #[$meta] )*
            $pub struct $name;
        }

        impl ::std::error::Error for $name {}
    }
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! unit_struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident;
    ) => {
        $crate::internal_unit_struct! {
            $( #[$meta] )*
            $pub struct $name;
        }
    }
}
