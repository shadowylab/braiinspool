// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

#[macro_use]
extern crate serde;

pub mod client;
pub mod model;
mod util;

pub use client::{Client, Error};
