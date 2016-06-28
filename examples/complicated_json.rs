#[macro_use]
extern crate log;
#[macro_use]
extern crate s_structured_log;
extern crate serde_json;

use s_structured_log::{JsonLogger, LoggerOutput, q};

fn main() {
    JsonLogger::init(LoggerOutput::Stderr, log::LogLevelFilter::Info);

    // use json_object!
    s_info!(json_object! {
        "Fruits" => json_object! {
            "on_the_table" => json_object! {
                "Apple" => 1,
                "Orange" => "two",
                "Grape" => 1.2
            },
            "in_the_basket" => ["Banana", "Strawberry"]
        },
        "Pets" => [
            json_object! {
                "name" => "Tama",
                "kind" => "cat",
                "age" => 3
            },
            json_object! {
                "name" => "Pochi",
                "kind" => "dog",
                "age" => 5
            }
        ]
    });

    // use json_format! and target with `json:` prefix.
    info!(target: &format!("json:{}", module_path!()),
          "{}",
          json_format! {
        "Fruits" => json_format! {
            "on_the_table" => json_format! {
                "Apple" => 1,
                "Orange" => q("two"),
                "Grape" => 1.2
            },
            "in_the_basket" => json_format![q("Banana"), q("Strawberry")]
        },
        "Pets" => json_format![
            json_format! {
                "name" => q("Tama"),
                "kind" => q("cat"),
                "age" => 3
            },
            json_format! {
                "name" => q("Pochi"),
                "kind" => q("dog"),
                "age" => 5
            }
        ]
    });

    // use json_format! and default target.
    info!("{}",
          json_format! {
        "Fruits" => json_format! {
            "on_the_table" => json_format! {
                "Apple" => 1,
                "Orange" => 2,
                "Grape" => 1.2
            },
            "in_the_basket" => json_format![q("Banana"), q("Strawberry")]
        },
        "Pets" => json_format![
            json_format! {
                "name" => q("Tama"),
                "kind" => q("cat")
            },
            json_format! {
                "name" => q("Pochi"),
                "kind" => q("dog")
            }
        ]
    });
}
