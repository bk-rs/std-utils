#![cfg(feature = "std")]

// cargo expand -p error-macro --verbose --test macro_enum

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_std_error() {
        use std::error::Error;

        //
        fn day(err: Box<dyn Error>) {
            assert!(err.downcast_ref::<Day>().is_some());
        }
        day(Day::Monday.into());

        //
        fn flash_message(err: Box<dyn Error>) {
            assert!(err.downcast_ref::<FlashMessage>().is_some());
        }
        flash_message(FlashMessage::Error("".into()).into());
    }
}
