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

pub enum LoggerOutput {
    Stdout,
    Stderr,
}

struct JsonLogger {
    filter: log::LogLevelFilter,
    output: LoggerOutput,
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

pub fn init(output: LoggerOutput, log_level: log::LogLevelFilter) {
    let logger = JsonLogger {
        filter: log_level,
        output: output,
    };

    log::set_logger(|max_log_level| {
            max_log_level.set(logger.filter);
            Box::new(logger)
        })
        .unwrap();
}

#[cfg(feature = "documentation")]
pub mod doc;

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
