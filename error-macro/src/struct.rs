// Ref https://learning-rust.github.io/docs/b2.structs.html
// Ref https://doc.rust-lang.org/rust-by-example/custom_types/structs.html

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
        $crate::internal_c_like_struct! {
            $( #[$meta] )*
            $pub struct $name {
                $( $field_pub $field_name: $field_ty ),*
            }
        }

        impl ::std::error::Error for $name {}
    };

    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident($( $inner_pub:vis $inner_ty:ty ),* $(,)? );
    ) => {
        $crate::internal_tuple_struct! {
            $( #[$meta] )*
            $pub struct $name($( $inner_pub $inner_ty ),*);
        }

        impl ::std::error::Error for $name {}
    };

    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident;
    ) => {
        $crate::internal_unit_struct! {
            $( #[$meta] )*
            $pub struct $name;
        }

        impl ::std::error::Error for $name {}
    };
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
        $crate::internal_c_like_struct! {
            $( #[$meta] )*
            $pub struct $name {
                $( $field_pub $field_name: $field_ty ),*
            }
        }
    };

    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident($( $inner_pub:vis $inner_ty:ty ),* $(,)? );
    ) => {
        $crate::internal_tuple_struct! {
            $( #[$meta] )*
            $pub struct $name($( $inner_pub $inner_ty ),*);
        }
    };

    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident;
    ) => {
        $crate::internal_unit_struct! {
            $( #[$meta] )*
            $pub struct $name;
        }
    };
}

//
#[macro_export(local_inner_macros)]
macro_rules! internal_c_like_struct {
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
