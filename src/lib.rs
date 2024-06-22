#![doc = include_str!("../README.md")]
pub mod client;
pub(crate) mod connector_errors;
#[doc(hidden)]
pub mod endpoints;
pub mod types;

pub use client::ConnectorClient;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Debug)]
#[doc(hidden)]
pub enum ResponseWraper {
    #[serde(rename = "result")]
    Result(Value),
    #[serde(rename = "error")]
    Error(Value),
}

#[cfg(feature = "blocking")]
pub mod blocking;
