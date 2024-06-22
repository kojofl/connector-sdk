#![doc = include_str!("../README.md")]
pub mod client;
pub(crate) mod connector_errors;
#[doc(hidden)]
pub mod endpoints;
pub mod types;

pub use client::ConnectorClient;

#[cfg(feature = "blocking")]
pub mod blocking;
