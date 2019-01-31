extern crate colored_json;
extern crate serde_json;

use colored_json::prelude::*;

fn main() -> ::std::result::Result<(), Box<::std::error::Error>> {
    #[cfg(windows)]
    let _enabled = colored_json::enable_ansi_support();

    println!(
        "{}",
        r#"{
              "array": [
                "ele1",
                "ele2"
              ],
              "float": 3.1415926,
              "integer": 4398798674962568,
              "string": "string"
           }
        "#
        .to_colored_json_auto()?
    );
    Ok(())
}
