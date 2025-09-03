#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::large_futures)]
#![warn(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod error;
pub mod model;
pub mod prelude;
mod util;
