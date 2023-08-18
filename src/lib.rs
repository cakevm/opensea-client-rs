#![warn(unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
//! An unofficial implementation of the Opensea V2 API in rust

/// This module contains the core client implementation.
pub mod client;

/// This module contains constants used by the client.
mod constants;

/// This module contains the core type definitions for the client.
pub mod types;

pub use client::{OpenSeaApiConfig, OpenSeaV2Client};

//XXX Suppress false positive unused_crate_dependencies warning
#[cfg(test)]
mod test {
    use tokio as _;
}
