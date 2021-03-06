//! # ddg: A DuckDuckGo Instant Answers wrapper library.
//!
//! This library provides a strongly typed wrapper around the DuckDuckGo Instant
//! Answers API. Most of the documentation comes from the
//! [DuckDuckGo Instant Answers API Documentation](https://duckduckgo.com/api)
//! This library comes with reqwest by default for convenience, however it can be
//! disabled. If disabled the library will fallback to hyper for `IntoUrl` so it
//! can be used with your own hyper client implementation.
//!
//! ### Example
//!
//! ```
//! use ddg::Query;
//! const APP_NAME: &'static str = "ddg_example_app";
//! // Search for Rust and we want to strip out any HTML content in the answers.
//! let query = Query::new("Rust", APP_NAME).no_html();
//!
//! let response = query.execute().unwrap();
//!
//! println!("{:?}", response);
//! ```

#![deny(missing_docs)]

#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

/// The Query struct, and it's Error enum.
pub mod query;
/// The Response, and all the related types.
pub mod response;

pub use query::Query;
pub use response::{RelatedTopic, Response, Type};

