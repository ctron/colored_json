use ansi_term::Style;
use serde_json::json;
use serde_json::ser::{CompactFormatter, PrettyFormatter};
use std::error::Error;
use std::io::Write;
use std::result::Result;
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
    println!("\n{}", s);

    return Ok(());
}

#[test]
fn test_writer() -> Result<(), Box<Error>> {
    let data = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });

    let mut writer: Vec<u8> = Vec::with_capacity(128);
    writer.write_all(b"\n")?;
    write_colored_json(&data, &mut writer)?;
    writer.write_all(b"\n")?;
    let s = unsafe { String::from_utf8_unchecked(writer) };
    println!("{}", s);
    return Ok(());
}

#[test]
fn test_styler() -> Result<(), Box<Error>> {
    let f = ColoredFormatter::with_styler(
        PrettyFormatter::new(),
        Styler {
            key: Color::Green.normal(),
            value: Colour::Blue.bold(),
            object: Style::new().bold(),
        },
    );

    println!(
        "\n{}",
        f.clone().to_colored_json(&json!({
          "name": "John Doe",
          "age": 43,
          "phones": [
            "+44 1234567",
            "+44 2345678"
          ]
        }))?
    );

    println!(
        "{}",
        f.to_colored_json(&json!({
        "name":"John", "age":31, "city":"New York"
    }))?
    );

    return Ok(());
}

#[test]
fn test_styler_compact() -> Result<(), Box<Error>> {
    let f = ColoredFormatter::with_styler(
        CompactFormatter {},
        Styler {
            key: Color::Green.normal(),
            value: Colour::Blue.bold(),
            object: Style::new().bold(),
        },
    );

    println!(
        "\n{}",
        f.clone().to_colored_json(&json!({
          "name": "John Doe",
          "age": 43,
          "phones": [
            "+44 1234567",
            "+44 2345678"
          ]
        }))?
    );

    println!(
        "{}",
        f.to_colored_json(&json!({
        "name":"John", "age":31, "city":"New York"
    }))?
    );

    return Ok(());
}
