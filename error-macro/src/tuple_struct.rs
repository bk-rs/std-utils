#[macro_export(local_inner_macros)]
macro_rules! internal_tuple_struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident($( $inner_pub:vis $inner_ty:ty ),* $(,)? );
    ) => {
        $( #[$meta] )*
        #[derive(Debug)]
        $pub struct $name($( $inner_pub $inner_ty ),*);

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
macro_rules! tuple_struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident($( $inner_pub:vis $inner_ty:ty ),* $(,)? );
    ) => {
        $crate::internal_tuple_struct! {
            $( #[$meta] )*
            $pub struct $name($( $inner_pub $inner_ty ),*);
        }

        impl ::std::error::Error for $name {}
    }
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! tuple_struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident($( $inner_pub:vis $inner_ty:ty ),* $(,)? );
    ) => {
        $crate::internal_tuple_struct! {
            $( #[$meta] )*
            $pub struct $name($( $inner_pub $inner_ty ),*);
        }
    }
}
