#![cfg(not(feature = "std"))]

// cargo expand -p error-macro --no-default-features  --verbose --test no_std

//
error_macro::r#struct! {
    pub struct CLikeStructError {
        pub code: u64
    }
}

//
error_macro::r#struct! {
    pub struct TupleStructError(u64);
}

//
error_macro::r#struct! {
    pub struct UnitStructError;
}
