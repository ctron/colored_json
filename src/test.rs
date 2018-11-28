use serde_json::json;
use std::error::Error;
use std::result::Result;
use std::io::stdout;
use std::io::Write;
use *;

#[test]
fn test_display_json_value() -> Result<(), Box<Error>> {
    let data = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });

    let s = to_colored_json(&data)?;
    println!("{}", s);

    return Ok(());
}

#[test]
fn test_stdout() -> Result<(), Box<Error>> {
    let data = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });

    let mut stdout = stdout();
    write_colored_json(&data, &mut stdout)?;
    stdout.write_all(b"\n")?;
    return Ok(());
}
