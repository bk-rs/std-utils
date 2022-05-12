#![cfg(feature = "std")]

// cargo expand -p error-macro --verbose --test macro_struct

//
error_macro::r#struct! {
    pub struct CLikeFooError {
        pub code: u64
    }
}

error_macro::r#struct! {
    #[derive(Clone)]
    pub struct CLikeBarError {
        pub code: u64,
        pub desc: Box<str>,
    }
}

//
error_macro::r#struct! {
    pub struct TupleFooError(u64);
}

error_macro::r#struct! {
    #[derive(Clone)]
    pub struct TupleBarError(pub u64, pub Box<str>);
}

//
error_macro::r#struct! {
    pub struct UnitFooError;
}

error_macro::r#struct! {
    #[derive(Copy, Clone)]
    pub struct UnitBarError;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_std_error() {
        use std::error::Error;

        //
        fn c_like(err: Box<dyn Error>) {
            assert!(err.downcast_ref::<CLikeFooError>().is_some());
        }
        c_like(CLikeFooError { code: 0 }.into());

        //
        fn tuple(err: Box<dyn Error>) {
            assert!(err.downcast_ref::<TupleFooError>().is_some());
        }
        tuple(TupleFooError(0).into());

        //
        fn unit(err: Box<dyn Error>) {
            assert!(err.downcast_ref::<UnitFooError>().is_some());
        }
        unit(UnitFooError.into());
    }
}
