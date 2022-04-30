#![cfg_attr(not(feature = "std"), no_std)]

pub extern crate alloc;

//
#[macro_export]
macro_rules! impl_partial_eq_str_for_display {
    ($str:ty, $display:ty) => {
        #[allow(unused_lifetimes)]
        impl<'a> ::core::cmp::PartialEq<$str> for $display {
            fn eq(&self, other: &$str) -> bool {
                ::core::cmp::PartialEq::eq(&$crate::alloc::format!("{}", self)[..], &other[..])
            }
        }
    };
}
