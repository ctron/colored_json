# Colored JSON output for Rust [![Travis (.org)](https://img.shields.io/travis/ctron/colored_json.svg)](https://travis-ci.org/ctron/colored_json) [![Crates.io](https://img.shields.io/crates/v/colored_json.svg)](https://crates.io/crates/colored_json)

Also see:
 * https: https://crates.io/crates/colored_json

## Using

Add it to your project:

~~~toml
[dependencies]
colored_json = "0.1"
~~~

And then color your JSON output:

~~~rust
use colored_json::to_colored_json;

use serde_json::value::Value;
use serde_json::error;

pub fn display_json_value(value: &Value) -> std::result::Result<(), error::Error> {

    println!("{}", to_colored_json(value)?);

    Ok(())

}
~~~

Or directly write it out:

~~~rust
let mut out = stdout();

{
    let mut out = out.lock();
    write_colored_json(value, & mut out)?
}

out.flush()?;
~~~

