//! # Basic usage
//!
//! Cargo.toml:
//!
//! ```toml
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
//!
//!     // Output:
//!     // {"level":"INFO","meta":{"target":"json:basic","location":{"module_path":"basic","file":"examples\\basic.rs","line":20}},"value":{"info_key1":1,"info_key2":"value2"}}
//!     // {"level":"WARN","meta":{"target":"json:basic","location":{"module_path":"basic","file":"examples\\basic.rs","line":24}},"value":{"warn_key1":1,"warn_key2":"value2"}}
//!     // {"level":"ERROR","meta":{"target":"json:basic","location":{"module_path":"basic","file":"examples\\basic.rs","line":28}},"value":{"error_key1":1,"error_key2":"value2"}}
//!     // {"level":"ERROR","meta":{"target":"basic","location":{"module_path":"basic","file":"examples\\basic.rs","line":38}},"value":"{\"error_key1\":1,\"error_key2\":\"value2\",\"error_key3\":[\"value3\",4]}"}
//! }
//! ```
//!
//! # More complicated JSON
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
//!                 "Grape" => 1.2
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
//!                 "Grape" => 1.2
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
//!                 "Grape" => 1.2
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
//!
//!     // Output:
//!     // {"level":"INFO","meta":{"target":"json:complicated_json","location":{"module_path":"complicated_json","file":"examples\\complicated_json.rs","line":13}},"value":{"Fruits":{"in_the_basket":["Banana","Strawberry"],"on_the_table":{"Apple":1,"Grape":1.2,"Orange":"two"}},"Pets":[{"age":3,"kind":"cat","name":"Tama"},{"age":5,"kind":"dog","name":"Pochi"}]}}
//!     // {"level":"INFO","meta":{"target":"json:complicated_json","location":{"module_path":"complicated_json","file":"examples\\complicated_json.rs","line":37}},"value":{"Fruits":{"on_the_table":{"Apple":1,"Orange":"two","Grape":1.2},"in_the_basket":["Banana","Strawberry"]},"Pets":[{"name":"Tama","kind":"cat","age":3},{"name":"Pochi","kind":"dog","age":5}]}}
//!     // {"level":"INFO","meta":{"target":"complicated_json","location":{"module_path":"complicated_json","file":"examples\\complicated_json.rs","line":63}},"value":"{\"Fruits\":{\"on_the_table\":{\"Apple\":1,\"Orange\":2,\"Grape\":1.2},\"in_the_basket\":[\"Banana\",\"Strawberry\"]},\"Pets\":[{\"name\":\"Tama\",\"kind\":\"cat\"},{\"name\":\"Pochi\",\"kind\":\"dog\"}]}"}
//!
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

#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;

use serde::{Serialize, Serializer};
use serde_json::to_string;
use std::fmt::{Debug, Display};
use std::io::{Write, stderr, stdout};

#[macro_export]
macro_rules! json_object {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            use serde_json::{Value, to_value, Map};
            let mut m: Map<String, Value> = Map::new();
            $(
                m.insert($key.to_owned(), to_value(&$value));
            )*
            m
        }
    }
}

#[macro_export]
macro_rules! json_format {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut s = String::with_capacity(128);
            s.push('{');
            $(
                s.push_str(&format!(r#""{}":{},"#, $key, $value));
            )*
            let _ = s.pop();
            s.push('}');
            s
        }
    };
    ( $( $value:expr ),* ) => {
        {
            let mut s = String::with_capacity(128);
            s.push('[');
            $(
                s.push_str(&format!("{},", $value));
            )*
            let _ = s.pop();
            s.push(']');
            s
        }
    }
}

/// Make a quoted and escaped string for JSON.
///
/// ```
/// use s_structured_log::q;
///
/// let quoted = q("abc");
/// assert_eq!(quoted, "\"abc\"");
/// ```
///
/// ```
/// use s_structured_log::q;
///
/// let x = "ab\ncdef\x02\x03猫\"bbb🐈";
/// let expected = r#""ab\ncdef\u0002\u0003猫\"bbb🐈""#;
/// assert_eq!(q(x), expected.to_owned());
/// ```
pub fn q<T: Display + ?Sized>(x: &T) -> String {
    format!("\"{}\"", escape_str(&x.to_string()))
}

pub trait StructuredLog {
    fn slog(&self) -> String;
}

#[inline]
pub fn serialize<T>(value: &T) -> String
    where T: StructuredLog
{
    value.slog()
}

#[derive(Debug)]
pub struct SLogJson<'a, T: 'a>(&'a T);

impl<'a, T: Serialize> SLogJson<'a, T> {
    pub fn new<'b>(value: &'b T) -> SLogJson<'b, T> {
        SLogJson(value)
    }
}

impl<'a, T: Serialize> Serialize for SLogJson<'a, T> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        self.0.serialize(serializer)
    }
}

impl<'a, T: Serialize + Debug> StructuredLog for SLogJson<'a, T> {
    fn slog(&self) -> String {
        to_string(self).unwrap_or_else(|err| {
            json_format! {
                "format_error" => q(&format!("{:?}", err)),
                "value" => q(&format!("{:?}", self))
            }
        })
    }
}

#[macro_export]
macro_rules! s_error {
    (target: $target:expr, $value:expr) => {
        {
            error!(target: &format!("json:{}", $target), "{}", $crate::serialize(&$crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_error!(target: module_path!(), $value);
        }
    };
}

#[macro_export]
macro_rules! s_warn {
    (target: $target:expr, $value:expr) => {
        {
            warn!(target: &format!("json:{}", $target), "{}", $crate::serialize(&$crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_warn!(target: module_path!(), $value);
        }
    };
}

#[macro_export]
macro_rules! s_info {
    (target: $target:expr, $value:expr) => {
        {
            info!(target: &format!("json:{}", $target), "{}", $crate::serialize(&$crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_info!(target: module_path!(), $value);
        }
    };
}

#[macro_export]
macro_rules! s_debug {
    (target: $target:expr, $value:expr) => {
        {
            debug!(target: &format!("json:{}", $target), "{}", $crate::serialize(&$crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_debug!(target: module_path!(), $value);
        }
    };
}

#[macro_export]
macro_rules! s_trace {
    (target: $target:expr, $value:expr) => {
        {
            trace!(target: &format!("json:{}", $target), "{}", $crate::serialize(&$crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_trace!(target: module_path!(), $value);
        }
    };
}

/// Escape characters for JSON.
///
/// ```
/// use s_structured_log::escape_str;
///
/// let x = "ab\ncdef\x02\x03猫\"bbb🐈";
/// let expected = r#"ab\ncdef\u0002\u0003猫\"bbb🐈"#;
/// assert_eq!(escape_str(x), expected.to_owned());
/// ```
pub fn escape_str(x: &str) -> String {
    let mut v = Vec::new();
    let mut l = 0;
    let bytes = x.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        let escaped = match *b {
            b'\x08' => r"\b".to_owned(),
            b'\x09' => r"\t".to_owned(),
            b'\x0a' => r"\n".to_owned(),
            b'\x0c' => r"\f".to_owned(),
            b'\x0d' => r"\r".to_owned(),
            b'\\' => r"\\".to_owned(),
            b'"' => r#"\""#.to_owned(),
            a if a < b'\x20' => format!(r"\u{:04x}", a),
            _ => {
                continue;
            }
        };

        if l < i {
            v.extend_from_slice(&bytes[l..i]);
        }

        v.extend_from_slice(escaped.as_bytes());

        l = i + 1;
    }

    v.extend_from_slice(&bytes[l..]);

    String::from_utf8(v).unwrap()
}

/// This enum indicates where the JsonLogger output to.
pub enum LoggerOutput {
    Stdout,
    Stderr,
}

/// This logger is a implementation for `log::Log` trait.
pub struct JsonLogger {
    filter: log::LogLevelFilter,
    output: LoggerOutput,
}

impl JsonLogger {
    /// ```
    /// #[macro_use]
    /// extern crate log;
    /// #[macro_use]
    /// extern crate s_structured_log;
    /// extern crate serde_json;
    ///
    /// use log::LogLevelFilter;
    /// use s_structured_log::{JsonLogger, LoggerOutput};
    ///
    /// fn main() {
    ///     JsonLogger::init(LoggerOutput::Stderr, LogLevelFilter::Info);
    ///
    ///     s_info!(json_object! {
    ///         "key" => "value"
    ///     });
    /// }
    /// ```
    pub fn init(output: LoggerOutput, filter: log::LogLevelFilter) {
        let logger = JsonLogger {
            filter: filter,
            output: output,
        };
        log::set_logger(|max_log_level| {
                max_log_level.set(logger.filter);
                Box::new(logger)
            })
            .unwrap();
    }
}

impl log::Log for JsonLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= self.filter
    }

    fn log(&self, record: &log::LogRecord) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let json = json_format! {
            "level" => q(&record.level()),
            "meta" => json_format! {
                "target" => q(&record.target()),
                "location" => json_format! {
                    "module_path" => q(&record.location().module_path()),
                    "file" => q(&record.location().file()),
                    "line" => record.location().line()
                }
            },
            "value" => if record.target().starts_with("json:") {
                format!("{}", record.args())
            } else {
                q(&record.args().to_string())
            }
        };

        let _ = match self.output {
            LoggerOutput::Stderr => writeln!(stderr(), "{}", json),
            LoggerOutput::Stdout => writeln!(stdout(), "{}", json),
        };
    }
}

#[cfg(test)]
mod tests {
    use q;

    #[test]
    fn simple_logger() {}

    #[test]
    fn only_escape_chars() {
        let x = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\\\"";
        let expected = r#"\u0000\u0001\u0002\u0003\u0004\u0005\u0006\u0007\b\t\n\u000b\f\r\u000e\u000f\u0010\u0011\u0012\u0013\u0014\u0015\u0016\u0017\u0018\u0019\\\""#.to_owned();
        assert_eq!(::escape_str(x), expected);
    }

    #[test]
    fn no_escape_chars() {
        let x = "abcdefghijklmn猫🐈";
        assert_eq!(::escape_str(x), x.to_owned());
    }

    #[test]
    fn random_escape_chars() {
        let x = "ab\ncdef\x02\x03猫\"bbb🐈";
        let expected = r#"ab\ncdef\u0002\u0003猫\"bbb🐈"#;
        assert_eq!(::escape_str(x), expected.to_owned());
    }

    #[test]
    fn json_format() {
        let obj = json_format! {
            "key1" => q("value1"),
            "key2" => 1
        };

        let array = json_format![q("value1"), 1];

        assert_eq!(obj, r#"{"key1":"value1","key2":1}"#);
        assert_eq!(array, r#"["value1",1]"#);
    }
}
