#![doc = include_str!("../README.md")]

#[macro_use]
extern crate serde;

pub mod client;
pub mod model;
mod util;

pub use client::{Client, Error};
pub use model::{DailyRewards, PoolStats, UserProfile, Workers};
