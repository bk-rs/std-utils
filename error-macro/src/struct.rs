#[macro_export(local_inner_macros)]
macro_rules! internal_struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident {
            $( $field_pub:vis $field_name:ident: $field_ty:ty ),* $(,)?
        }
    ) => {
        $( #[$meta] )*
        #[derive(Debug)]
        $pub struct $name {
            $( $field_pub $field_name: $field_ty ),*
        }

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
macro_rules! r#struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident {
            $( $field_pub:vis $field_name:ident: $field_ty:ty ),* $(,)?
        }
    ) => {
        $crate::internal_struct! {
            $( #[$meta] )*
            $pub struct $name {
                $( $field_pub $field_name: $field_ty ),*
            }
        }

        impl ::std::error::Error for $name {}
    }
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! r#struct {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident {
            $( $field_pub:vis $field_name:ident: $field_ty:ty ),* $(,)?
        }
    ) => {
        $crate::internal_struct! {
            $( #[$meta] )*
            $pub struct $name {
                $( $field_pub $field_name: $field_ty ),*
            }
        }
    }
}
