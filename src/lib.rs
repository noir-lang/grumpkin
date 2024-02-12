#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

#[cfg(feature = "curve")]
pub mod curves;

pub mod fields;

#[cfg(feature = "curve")]
pub use curves::*;

pub use fields::*;
