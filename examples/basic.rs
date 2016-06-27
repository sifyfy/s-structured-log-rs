#[macro_use]
extern crate log;
#[macro_use]
extern crate s_structured_log;
extern crate serde_json;

use s_structured_log::LoggerOutput;

fn main() {
    s_structured_log::init(LoggerOutput::Stdout, log::LogLevelFilter::Trace);

    s_trace!(json_object! {
        "trace_key1" => 1,
        "trace_key2" => "value2"
    });
    s_debug!(json_object! {
        "debug_key1" => 1,
        "debug_key2" => "value2"
    });
    s_info!(json_object! {
        "info_key1" => 1,
        "info_key2" => "value2"
    });
    s_warn!(json_object! {
        "warn_key1" => 1,
        "warn_key2" => "value2"
    });
    s_error!(json_object! {
        "error_key1" => 1,
        "error_key2" => "value2"
    });
}
