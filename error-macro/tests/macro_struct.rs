#![cfg(feature = "std")]

// cargo expand -p error-macro --verbose --test macro_struct

//
error_macro::r#struct! {
    pub struct FooError {
        pub code: u64
    }
}

error_macro::r#struct! {
    #[derive(Clone)]
    pub struct BarError {
        pub code: u64,
        pub desc: Box<str>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_std_error() {
        use std::error::Error;

        fn foo(err: Box<dyn Error>) {
            assert!(err.downcast_ref::<FooError>().is_some());
        }

        foo(FooError { code: 0 }.into());
    }
}
