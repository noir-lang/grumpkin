#[cfg(feature = "scalar_field")]
pub mod fr;

#[cfg(feature = "scalar_field")]
pub use fr::*;

#[cfg(feature = "curve")]
pub mod fq;

#[cfg(feature = "curve")]
pub use fq::*;
