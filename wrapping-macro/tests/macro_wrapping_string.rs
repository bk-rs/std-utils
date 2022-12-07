#![cfg(feature = "std")]

// cargo expand -p wrapping-macro --verbose --test macro_wrapping_string

//
wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct StringWrapper(pub String);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_methods_and_impl() {
        // From
        let _ = StringWrapper::from(String::from("foo"));
        let _ = StringWrapper::from(&String::from("foo"));
        let _ = StringWrapper::from("foo");
        let _ = StringWrapper::from(Box::<str>::from("foo"));
        let _ = StringWrapper::from(&Box::<str>::from("foo"));

        // Display and FromStr
        let w = "foo".parse::<StringWrapper>().unwrap();
        assert_eq!(format!("{w}"), "foo");
    }
}
