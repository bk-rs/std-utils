//
#[macro_export]
macro_rules! wrapping {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident($inner_pub:vis $inner_ty:ty);
    ) => {
        $( #[$meta] )*
        $pub struct $name($inner_pub $inner_ty);

        impl $name {
            pub fn from_inner(inner: $inner_ty) -> Self {
                Self(inner)
            }

            pub fn inner(&self) -> &$inner_ty {
                &self.0
            }

            pub fn into_inner(self) -> $inner_ty {
                self.0
            }
        }

        impl ::core::convert::From<$inner_ty> for $name {
            fn from(v: $inner_ty) -> Self {
                Self(v)
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = $inner_ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
}

//
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! wrapping_string {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident($inner_pub:vis $inner_ty:ty);
    ) => {
        wrapping_macro::wrapping!{
            $( #[$meta] )*
            $pub struct $name($inner_pub $inner_ty);
        }

        impl ::core::convert::From<&$crate::alloc::string::String> for $name {
            fn from(v: &$crate::alloc::string::String) -> Self {
                Self(v.to_owned())
            }
        }
        impl ::core::convert::From<&::core::primitive::str> for $name {
            fn from(v: &::core::primitive::str) -> Self {
                Self(v.into())
            }
        }
        impl ::core::convert::From<$crate::alloc::boxed::Box<::core::primitive::str>> for $name {
            fn from(v: $crate::alloc::boxed::Box<::core::primitive::str>) -> Self {
                Self(v.to_string())
            }
        }
        impl ::core::convert::From<&$crate::alloc::boxed::Box<::core::primitive::str>> for $name {
            fn from(v: &$crate::alloc::boxed::Box<::core::primitive::str>) -> Self {
                Self(v.to_string())
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = ::core::convert::Infallible;

            fn from_str(s: &::core::primitive::str) -> Result<Self, Self::Err> {
                Ok(Self(s.into()))
            }
        }
    }
}
