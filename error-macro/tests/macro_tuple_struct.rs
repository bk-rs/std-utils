#![cfg(feature = "std")]

// cargo expand -p error-macro --verbose --test macro_tuple_struct

//
error_macro::tuple_struct! {
    pub struct FooError(u64);
}

error_macro::tuple_struct! {
    #[derive(Clone)]
    pub struct BarError(pub u64, pub Box<str>);
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

        foo(FooError(0).into());
    }
}
