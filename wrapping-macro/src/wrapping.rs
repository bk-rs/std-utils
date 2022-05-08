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
