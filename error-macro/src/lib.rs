#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
pub extern crate alloc;

//
mod r#struct;
mod tuple_struct;
mod unit_struct;
