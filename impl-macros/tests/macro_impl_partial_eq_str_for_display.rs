#![cfg(feature = "std")]

// cargo expand --verbose --test macro_impl_partial_eq_str_for_display

use core::fmt;
use std::borrow::Cow;

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
impl_macros::impl_partial_eq_str_for_display!(String, Foo);
impl_macros::impl_partial_eq_str_for_display!(Cow<'a, str>, Foo);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        assert_eq!(Foo, "foo");
        assert_eq!(Foo, "foo".to_string());
        assert_eq!(Foo, Cow::Borrowed("foo"));
    }
}
