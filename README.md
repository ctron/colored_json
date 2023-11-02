# Colored JSON output for Rust [![ci](https://github.com/ctron/colored_json/actions/workflows/ci.yaml/badge.svg)](https://github.com/ctron/colored_json) [![docs.rs](https://img.shields.io/docsrs/colored_json)](https://docs.rs/colored_json/latest/colored_json/) [![Crates.io](https://img.shields.io/crates/v/colored_json.svg)](https://crates.io/crates/colored_json)

![Screenshot](Screenshot.png)

## Using

Add it to your project:

~~~toml
[dependencies]
colored_json = "4"
~~~

And then color your JSON output:

~~~rust
use colored_json::prelude::*;

fn main() -> ::std::result::Result<(), Box<::std::error::Error>> {
    println!(
        "{}",
        r#"
    {
        "array": [
            "ele1",
            "ele2"
        ],
        "float": 3.1415926,
        "integer": 4398798674962568,
        "string": "string"
    }
    "#.to_colored_json_auto()?
    );
    Ok(())
}
~~~

Or directly write it out:

~~~rust
use serde_json::{from_str, Value};
use std::io::stdout;
use std::io::Write;

pub fn main() -> ::std::result::Result<(), Box<::std::error::Error>> {
    let value: Value = from_str(r#"
        {
            "array": [
                "ele1",
                "ele2"
            ],
            "float": 3.1415926,
            "integer": 4398798674962568,
            "string": "string"
        }
    "#)?;
    let out = stdout();
    {
        let mut out = out.lock();
        colored_json::write_colored_json(&value, &mut out)?;
        out.flush()?;
    }
    Ok(())
}
~~~

