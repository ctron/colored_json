use serde_json::json;
use std::error::Error;
use std::io::Write;
use std::result::Result;
use crate::*;

#[test]
fn test_display_json_value() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

    let data = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });

    let s = to_colored_json_auto(&data)?;
    println!("\n{}", s);

    return Ok(());
}

#[test]
fn test_trait() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

    println!(
        "\n{}",
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

#[test]
fn test_trait_err() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

    eprintln!(
        "\n{}",
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
        .to_colored_json(ColorMode::Auto(Output::StdErr))?
    );
    Ok(())
}

#[test]
fn test_trait_color_off() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

    println!(
        "\n{}",
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
        .to_colored_json(ColorMode::Off)?
    );
    Ok(())
}

#[test]
fn test_trait_styler() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

    println!(
        "\n{}",
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
        .to_colored_json_with_styler(
            ColorMode::default().eval(),
            Styler {
                key: Color::Green.normal(),
                string_value: Colour::Blue.bold(),
                integer_value: Colour::Purple.bold(),
                float_value: Colour::Purple.italic(),
                object_brackets: Colour::Yellow.bold(),
                array_brackets: Colour::Cyan.bold(),
                ..Default::default()
            },
        )?
    );
    Ok(())
}

#[test]
fn test_trait_styler_color_off() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

    println!(
        "\n{}",
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
        .to_colored_json_with_styler(
            ColorMode::Off,
            Styler {
                key: Color::Green.normal(),
                string_value: Colour::Blue.bold(),
                integer_value: Colour::Purple.bold(),
                float_value: Colour::Purple.italic(),
                object_brackets: Colour::Yellow.bold(),
                array_brackets: Colour::Cyan.bold(),
                ..Default::default()
            },
        )?
    );
    Ok(())
}

#[test]
fn test_writer() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

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
    #[cfg(windows)]
    let _res = enable_ansi_support();

    let f = ColoredFormatter::with_styler(
        PrettyFormatter::new(),
        Styler {
            key: Color::Green.normal(),
            string_value: Colour::Blue.bold(),
            integer_value: Colour::Purple.bold(),
            float_value: Colour::Purple.italic(),
            object_brackets: Colour::Yellow.bold(),
            array_brackets: Colour::Cyan.bold(),
            ..Default::default()
        },
    );

    println!(
        "\n{}",
        f.clone().to_colored_json_auto(&json!({
          "string": "string",
          "integer": 4398798674962568u64,
          "float": 3.1415926,
          "array": [
            "ele1",
            "ele2"
          ]
        }))?
    );

    println!(
        "{}",
        f.to_colored_json_auto(&json!({
            "name":"John", "age":31, "city":"New York"
        }))?
    );

    return Ok(());
}

#[test]
fn test_styler_no_color() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

    let f = ColoredFormatter::with_styler(
        PrettyFormatter::new(),
        Styler {
            key: Color::Green.normal(),
            string_value: Colour::Blue.bold(),
            integer_value: Colour::Purple.bold(),
            float_value: Colour::Purple.italic(),
            object_brackets: Colour::Yellow.bold(),
            array_brackets: Colour::Cyan.bold(),
            ..Default::default()
        },
    );

    println!(
        "\n{}",
        f.clone().to_colored_json(
            &json!({
              "string": "string",
              "integer": 4398798674962568u64,
              "float": 3.1415926,
              "array": [
                "ele1",
                "ele2"
              ]
            }),
            ColorMode::Off,
        )?
    );

    println!(
        "{}",
        f.to_colored_json_auto(&json!({
            "name":"John", "age":31, "city":"New York"
        }))?
    );

    return Ok(());
}

#[test]
fn test_styler_compact() -> Result<(), Box<Error>> {
    #[cfg(windows)]
    let _res = enable_ansi_support();

    let f = ColoredFormatter::with_styler(
        CompactFormatter {},
        Styler {
            key: Color::Green.normal(),
            string_value: Colour::Blue.bold(),
            integer_value: Colour::Blue.bold(),
            ..Default::default()
        },
    );

    println!(
        "\n{}",
        f.clone().to_colored_json_auto(&json!({
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
        f.to_colored_json_auto(&json!({
            "name":"John", "age":31, "city":"New York"
        }))?
    );

    return Ok(());
}
