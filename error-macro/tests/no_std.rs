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

//
error_macro::r#enum! {
    enum Day {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday
    }
}

error_macro::r#enum! {
    enum FlashMessage {
        // Success, // A unit variant
        Warning{ category: i32, message: String }, // A struct variant
        Error(String) // A tuple variant
    }
}

error_macro::r#enum! {
    enum WebEvent {
        // An `enum` may either be `unit-like`,
        // PageLoad,
        // PageUnload,
        // like tuple structs,
        KeyPress(char),
        Paste(String),
        // or c-like structures.
        Click { x: i64, y: i64 },
    }
}
