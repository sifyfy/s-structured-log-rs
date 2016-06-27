#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;

use serde::{Serialize, Serializer};
use serde_json::to_string;
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

#[derive(Debug)]
pub struct SLogMetadata<'a> {
    pub target: &'a str,
    pub level: log::LogLevel,
}

#[derive(Debug)]
pub struct SLogLocation<'a> {
    pub module_path: &'a str,
    pub file: &'a str,
    pub line: u32,
}

#[derive(Debug)]
pub struct SLogInfo<'a> {
    pub metadata: SLogMetadata<'a>,
    pub location: SLogLocation<'a>,
}

impl<'a> SLogInfo<'a> {
    pub fn metadata(&self) -> &SLogMetadata {
        &self.metadata
    }

    pub fn location(&self) -> &SLogLocation {
        &self.location
    }
}

pub trait StructuredLog {
    fn slog(&self, info: &SLogInfo) -> String;
}

#[inline]
pub fn serialize<T>(value: &T, info: &SLogInfo) -> String
    where T: StructuredLog
{
    value.slog(info)
}

#[macro_export]
macro_rules! serialize {
    (target: $target:expr, level: $level:expr, $value:expr) => {
        {
            $crate::serialize(&$value, &$crate::SLogInfo {
                metadata: $crate::SLogMetadata {
                    target: $target,
                    level: $level
                },
                location: $crate::SLogLocation {
                    module_path: module_path!(),
                    file: file!(),
                    line: line!()
                }
            })
        }
    }
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

impl<'a, T: Serialize> StructuredLog for SLogJson<'a, T> {
    fn slog(&self, info: &SLogInfo) -> String {
        let obj = json_object! {
            "target" => info.metadata.target,
            "level" => info.metadata.level.to_string(),
            "location" => json_object! {
                "module_path" => info.location.module_path,
                "file" => info.location.file,
                "line" => info.location.line
            },
            "value" => self
        };
        to_string(&obj).unwrap_or_else(|err| {
            writeln!(stderr(), "{}, {:?}", err, obj).unwrap();

            let msg = "Serializing error occured in s_structured_log in serde::to_string. \
                       Details was written out to STDERR.";
            format!("{{ \"level\": \"{}\", \"value\": \"{}\" }}",
                    log::LogLevel::Error,
                    msg)
        })
    }
}

#[macro_export]
macro_rules! s_error {
    (target: $target:expr, $value:expr) => {
        {
            use log;
            trace!(target: $target, "{}", serialize!(target: $target, level: log::LogLevel::Error, $crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_error!(target: &format!("json:{}", module_path!()), $value);
        }
    };
}

#[macro_export]
macro_rules! s_warn {
    (target: $target:expr, $value:expr) => {
        {
            use log;
            trace!(target: $target, "{}", serialize!(target: $target, level: log::LogLevel::Warn, $crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_warn!(target: &format!("json:{}", module_path!()), $value);
        }
    };
}

#[macro_export]
macro_rules! s_info {
    (target: $target:expr, $value:expr) => {
        {
            use log;
            trace!(target: $target, "{}", serialize!(target: $target, level: log::LogLevel::Info, $crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_info!(target: &format!("json:{}", module_path!()), $value);
        }
    };
}

#[macro_export]
macro_rules! s_debug {
    (target: $target:expr, $value:expr) => {
        {
            use log;
            trace!(target: $target, "{}", serialize!(target: $target, level: log::LogLevel::Debug, $crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_debug!(target: &format!("json:{}", module_path!()), $value);
        }
    };
}

#[macro_export]
macro_rules! s_trace {
    (target: $target:expr, $value:expr) => {
        {
            use log;
            trace!(target: $target, "{}", serialize!(target: $target, level: log::LogLevel::Trace, $crate::SLogJson::new(&$value)));
        }
    };
    ($value:expr) => {
        {
            s_trace!(target: &format!("json:{}", module_path!()), $value);
        }
    };
}

fn escape_str(x: &str) -> String {
    let mut v = Vec::new();
    let mut l = 0;
    let bytes = x.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        let escaped = match *b {
            b'\x00' => r"\u0000",
            b'\x01' => r"\u0001",
            b'\x02' => r"\u0002",
            b'\x03' => r"\u0003",
            b'\x04' => r"\u0004",
            b'\x05' => r"\u0005",
            b'\x06' => r"\u0006",
            b'\x07' => r"\u0007",
            b'\x08' => r"\b",
            b'\x09' => r"\t",
            b'\x10' => r"\n",
            b'\x11' => r"\u0011",
            b'\x12' => r"\f",
            b'\x13' => r"\r",
            b'\x14' => r"\u0014",
            b'\x15' => r"\u0015",
            b'\x16' => r"\u0016",
            b'\x17' => r"\u0017",
            b'\x18' => r"\u0018",
            b'\x19' => r"\u0019",
            b'\\' => r"\\",
            b'"' => r#"\""#,
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

        let location = format!(r#"{{"module_path":"{}","file":"{}","line":{}}}"#,
                               record.location().module_path(),
                               record.location().file(),
                               record.location().line());

        let meta = format!(r#"{{"target":"{}","location":{}}}"#,
                           record.target(),
                           location);

        let value = if record.target().starts_with("json:") {
            format!("{}", record.args())
        } else {
            format!(r#""{}""#, escape_str(&record.args().to_string()))
        };

        let json = format!(r#"{{"level":"{}","meta":{},"value":{}}}"#,
                           record.level().to_string(),
                           meta,
                           value);

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
    #[test]
    fn simple_logger() {}

    #[test]
    fn escape_str() {
        let x = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\\\"";
        let expected = r#"\u0000\u0001\u0002\u0003\u0004\u0005\u0006\u0007\b\t\n\u0011\f\r\u0014\u0015\u0016\u0017\u0018\u0019\\\""#.to_owned();
        assert_eq!(::escape_str(x), expected);
    }
}
