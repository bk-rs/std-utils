#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
pub extern crate alloc;

#[cfg(feature = "alloc")]
mod alloc_;
