#![cfg(feature = "alloc")]

// cargo expand -p impl-macros --verbose --test macro_impl_partial_eq_str_for_display

extern crate alloc;

use core::fmt;

//
#[derive(Debug)]
struct Foo;

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "foo")
    }
}

impl_macros::impl_partial_eq_str_for_display!(str, Foo);
impl_macros::impl_partial_eq_str_for_display!(&'a str, Foo);
impl_macros::impl_partial_eq_str_for_display!(alloc::string::String, Foo);
#[cfg(feature = "std")]
impl_macros::impl_partial_eq_str_for_display!(std::borrow::Cow<'a, str>, Foo);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        assert_eq!(Foo, "foo");
        assert_eq!(Foo, alloc::string::String::from("foo"));
        #[cfg(feature = "std")]
        assert_eq!(Foo, std::borrow::Cow::Borrowed("foo"));
    }
}
