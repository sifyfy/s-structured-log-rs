//! Document
//!
//! # Basic usage
//!
//! Cargo.toml:
//!
//! ```
//! ...
//!
//! [dependencies]
//! log = "*"
//! serde = "*"
//! serde_json = "*"
//! s-structured-log = "*"
//! ```
//!
//! main.rs:
//!
//! ```
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate s_structured_log;
//! extern crate serde_json;
//!
//! use s_structured_log::{JsonLogger, LoggerOutput, q};
//!
//! fn main() {
//!     JsonLogger::init(LoggerOutput::Stdout, log::LogLevelFilter::Info);
//!
//!     s_trace!(json_object! {
//!         "trace_key1" => 1,
//!         "trace_key2" => "value2"
//!     });
//!     s_debug!(json_object! {
//!         "debug_key1" => 1,
//!         "debug_key2" => "value2"
//!     });
//!     s_info!(json_object! {
//!         "info_key1" => 1,
//!         "info_key2" => "value2"
//!     });
//!     s_warn!(json_object! {
//!         "warn_key1" => 1,
//!         "warn_key2" => "value2"
//!     });
//!     s_error!(json_object! {
//!         "error_key1" => 1,
//!         "error_key2" => "value2"
//!     });
//!
//!     trace!("{:?}",
//!            json_object! {
//!         "trace_key1" => 1,
//!         "trace_key2" => "value2"
//!     });
//!     error!("{}",
//!            json_format! {
//!         "error_key1" => 1,
//!         "error_key2" => q("value2"),
//!         "error_key3" => json_format![q("value3"),4]
//!     });
//! }
//! ```
//!
//! ## More complicated JSON
//!
//! ```
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate s_structured_log;
//! extern crate serde_json;
//!
//! use s_structured_log::{JsonLogger, LoggerOutput, q};
//!
//! fn main() {
//!     JsonLogger::init(LoggerOutput::Stderr, log::LogLevelFilter::Info);
//!
//!     // use json_object!
//!     s_info!(json_object! {
//!         "Fruits" => json_object! {
//!             "on_the_table" => json_object! {
//!                 "Apple" => 1,
//!                 "Orange" => "two",
//!                 "Grape" => 1.0
//!             },
//!             "in_the_basket" => ["Banana", "Strawberry"]
//!         },
//!         "Pets" => [
//!             json_object! {
//!                 "name" => "Tama",
//!                 "kind" => "cat",
//!                 "age" => 3
//!             },
//!             json_object! {
//!                 "name" => "Pochi",
//!                 "kind" => "dog",
//!                 "age" => 5
//!             }
//!         ]
//!     });
//!
//!     // use json_format! and target with `json:` prefix.
//!     info!(target: &format!("json:{}", module_path!()),
//!           "{}",
//!           json_format! {
//!         "Fruits" => json_format! {
//!             "on_the_table" => json_format! {
//!                 "Apple" => 1,
//!                 "Orange" => q("two"),
//!                 "Grape" => 1.0
//!             },
//!             "in_the_basket" => json_format![q("Banana"), q("Strawberry")]
//!         },
//!         "Pets" => json_format![
//!             json_format! {
//!                 "name" => q("Tama"),
//!                 "kind" => q("cat"),
//!                 "age" => 3
//!             },
//!             json_format! {
//!                 "name" => q("Pochi"),
//!                 "kind" => q("dog"),
//!                 "age" => 5
//!             }
//!         ]
//!     });
//!
//!     // use json_format! and default target.
//!     info!("{}",
//!           json_format! {
//!         "Fruits" => json_format! {
//!             "on_the_table" => json_format! {
//!                 "Apple" => 1,
//!                 "Orange" => 2,
//!                 "Grape" => 1
//!             },
//!             "in_the_basket" => json_format![q("Banana"), q("Strawberry")]
//!         },
//!         "Pets" => json_format![
//!             json_format! {
//!                 "name" => q("Tama"),
//!                 "kind" => q("cat")
//!             },
//!             json_format! {
//!                 "name" => q("Pochi"),
//!                 "kind" => q("dog")
//!             }
//!         ]
//!     });
//! }
//! ```
//!
//! The `json_object!` macro make `serde_json::Map` object.
//! Values are required `serde::Serialize` implement.
//!
//! The `json_format!` macro make JSON text directly.
//! Values are required `std::fmt::Display` implement.
//! All Text values are required to add double quote manually
//! because `json_format` don't add double quote to values automatically.
//!
//! `json:` prefix is a tag for indicate to JsonLogger that input text is JSON.
//!
