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
            format!("{{ \"level\": \"{}\", \"value\": \"{}\" }}", log::LogLevel::Error, msg)
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
            s_error!(target: module_path!(), $value);
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
            s_warn!(target: module_path!(), $value);
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
            s_info!(target: module_path!(), $value);
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
            s_debug!(target: module_path!(), $value);
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
            s_trace!(target: module_path!(), $value);
        }
    };
}

pub enum LoggerOutput {
    Stdout,
    Stderr,
}

struct SimpleLogger {
    filter: log::LogLevelFilter,
    output: LoggerOutput,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= self.filter
    }

    fn log(&self, record: &log::LogRecord) {
        if !self.enabled(record.metadata()) {
            return;
        }

        match self.output {
            LoggerOutput::Stderr => writeln!(stderr(), "{}", record.args()).unwrap(),
            LoggerOutput::Stdout => writeln!(stdout(), "{}", record.args()).unwrap(),
        }
    }
}

pub fn init(output: LoggerOutput, log_level: log::LogLevelFilter) {
    let logger = SimpleLogger {
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
}
