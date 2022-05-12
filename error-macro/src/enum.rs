// Ref https://learning-rust.github.io/docs/b3.enums.html
// Ref https://doc.rust-lang.org/rust-by-example/custom_types/enum.html

//
#[cfg(feature = "std")]
#[macro_export]
macro_rules! r#enum {
    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $( $variant_name:ident ),+ $(,)?
        }
    ) => {
        $crate::internal_enum! {
            $( #[$meta] )*
            $pub enum $name {
                $( $variant_name ),+
            }
        }

        impl ::std::error::Error for $name {}
    };

    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $( $variant_name:ident $( $variant_data:tt )? ),+ $(,)?
        }
    ) => {
        $crate::internal_enum! {
            $( #[$meta] )*
            $pub enum $name {
                $( $variant_name $( $variant_data )? ),+
            }
        }

        impl ::std::error::Error for $name {}
    };
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! r#enum {
    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $( $variant_name:ident ),+ $(,)?
        }
    ) => {
        $crate::internal_enum! {
            $( #[$meta] )*
            $pub enum $name {
                $( $variant_name ),+
            }
        }
    };

    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $( $variant_name:ident $( $variant_data:tt )? ),+ $(,)?
        }
    ) => {
        $crate::internal_enum! {
            $( #[$meta] )*
            $pub enum $name {
                $( $variant_name $( $variant_data )? ),+
            }
        }
    };
}

//
#[macro_export(local_inner_macros)]
macro_rules! internal_enum {
    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $( $variant_name:ident ),+ $(,)?
        }
    ) => {
        $( #[$meta] )*
        #[derive(Debug)]
        $pub enum $name {
            $( $variant_name ),+
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{:?}", self)
            }
        }
    };

    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $( $variant_name:ident $( $variant_data:tt )? ),+ $(,)?
        }
    ) => {
        $( #[$meta] )*
        #[derive(Debug)]
        $pub enum $name {
            $( $variant_name $( $variant_data )? ),+
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{:?}", self)
            }
        }
    }
}
