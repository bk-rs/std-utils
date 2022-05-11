// cargo expand -p error-macro --verbose --test macro_unit_struct

//
error_macro::unit_struct! {
    pub struct FooError;
}

error_macro::unit_struct! {
    #[derive(Copy, Clone)]
    pub struct BarError;
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_std_error() {
        use std::error::Error;

        fn foo(err: Box<dyn Error>) {
            assert!(err.downcast_ref::<FooError>().is_some());
        }

        foo(FooError.into());
    }
}
