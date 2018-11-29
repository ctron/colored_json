/*!
colored_json crate to output colored serde json with ANSI terminal escape codes

# Examples

```rust
    extern crate serde_json;
    extern crate colored_json;

    use serde_json::json;

    use colored_json::to_colored_json;

    let s = to_colored_json(&json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    })).unwrap();
    println!("{}", s);
```

With a custom color style:

```rust
    extern crate serde_json;
    extern crate colored_json;

    use serde_json::json;
    use serde_json::ser::CompactFormatter;

    use colored_json::{ColoredFormatter, Styler, Style};
    use colored_json::Colour::{Green, Blue};

    let f = ColoredFormatter::with_styler(
        CompactFormatter {},
        Styler {
            key: Green.normal(),
            value: Blue.bold(),
            object: Style::new().bold(),
        },
    );

    println!(
        "{}",
        f.clone().to_colored_json(&json!({
          "name": "John Doe",
          "age": 43,
          "phones": [
            "+44 1234567",
            "+44 2345678"
          ]
        })).unwrap()
    );

    println!(
        "{}",
        f.to_colored_json(&json!({
        "name":"John", "age":31, "city":"New York"
    })).unwrap()
    );
```

!*/

extern crate ansi_term;
extern crate serde;
extern crate serde_json;

pub use ansi_term::Colour;
pub use ansi_term::Style;
use serde::Serialize;
use serde_json::ser::{Formatter, PrettyFormatter};
use serde_json::value::Value;

use std::io;

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct Styler {
    pub key: Style,
    pub value: Style,
    pub object: Style,
}

impl Default for Styler {
    fn default() -> Styler {
        Styler {
            key: Style::new().fg(Colour::Blue).bold(),
            value: Style::new().fg(Colour::Green),
            object: Style::new().bold(),
        }
    }
}

#[derive(Clone)]
pub struct ColoredFormatter<F>
where
    F: Formatter,
{
    formatter: F,
    styler: Styler,
    colorizer: Vec<Style>,
}

impl<F> ColoredFormatter<F>
where
    F: Formatter,
{
    pub fn new(formatter: F) -> Self {
        return ColoredFormatter {
            formatter,
            styler: Styler::default(),
            colorizer: Vec::new(),
        };
    }

    pub fn with_styler(formatter: F, styler: Styler) -> Self {
        return ColoredFormatter {
            formatter,
            styler: styler,
            colorizer: Vec::new(),
        };
    }

    pub fn to_colored_json(self, value: &Value) -> serde_json::Result<String> {
        let mut writer: Vec<u8> = Vec::with_capacity(128);

        self.write_colored_json(value, &mut writer)?;

        return Ok(String::from_utf8_lossy(&writer).to_string());
    }

    pub fn write_colored_json<W>(
        self,
        value: &Value,
        writer: &mut W,
    ) -> std::result::Result<(), serde_json::Error>
    where
        W: io::Write,
    {
        let mut serializer = serde_json::Serializer::with_formatter(writer, self);

        return value.serialize(&mut serializer);
    }
}

fn colored<W: ?Sized, H>(writer: &mut W, colorizer: Option<Style>, mut handler: H) -> io::Result<()>
where
    W: io::Write,
    H: FnMut(&mut Vec<u8>) -> io::Result<()>,
{
    let mut w: Vec<u8> = Vec::with_capacity(128);
    handler(&mut w)?;
    let s = String::from_utf8_lossy(&w);

    let out = match colorizer {
        Some(c) => c.paint(s).to_string(),
        None => s.to_string(),
    };
    Ok(writer.write_all(out.as_bytes())?)
}

impl<F> Formatter for ColoredFormatter<F>
where
    F: Formatter,
{
    fn write_null<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_null(writer)
    }

    fn write_bool<W: ?Sized>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_bool(writer, value)
    }

    fn write_i8<W: ?Sized>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_i8(writer, value)
    }

    fn write_i16<W: ?Sized>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_i16(writer, value)
    }

    fn write_i32<W: ?Sized>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_i32(writer, value)
    }

    fn write_i64<W: ?Sized>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_i64(writer, value)
    }

    fn write_u8<W: ?Sized>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_u8(writer, value)
    }

    fn write_u16<W: ?Sized>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_u16(writer, value)
    }

    fn write_u32<W: ?Sized>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_u32(writer, value)
    }

    fn write_u64<W: ?Sized>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_u64(writer, value)
    }

    fn write_f32<W: ?Sized>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_f32(writer, value)
    }

    fn write_f64<W: ?Sized>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_f64(writer, value)
    }

    fn write_number_str<W: ?Sized>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_number_str(writer, value)
    }

    fn begin_string<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.colorizer.last().cloned(), |w| {
            self.formatter.begin_string(w)
        })
    }

    fn end_string<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.colorizer.last().cloned(), |w| {
            self.formatter.end_string(w)
        })
    }

    fn write_string_fragment<W: ?Sized>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.colorizer.last().cloned(), |w| {
            self.formatter.write_string_fragment(w, fragment)
        })
    }

    fn begin_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.begin_array(writer)
    }

    fn end_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.end_array(writer)
    }

    fn begin_array_value<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.begin_array_value(writer, first)
    }

    fn end_array_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.end_array_value(writer)
    }

    fn begin_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, Some(self.styler.object), |w| {
            self.formatter.begin_object(w)
        })
    }

    fn end_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, Some(self.styler.object), |w| {
            self.formatter.end_object(w)
        })
    }

    fn begin_object_key<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        self.colorizer.push(self.styler.key);
        self.formatter.begin_object_key(writer, first)
    }

    fn end_object_key<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.end_object_key(writer)?;
        self.colorizer.pop();
        Ok(())
    }

    fn begin_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.colorizer.push(self.styler.value);
        self.formatter.begin_object_value(writer)
    }

    fn end_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.end_object_value(writer)?;
        self.colorizer.pop();
        Ok(())
    }

    fn write_raw_fragment<W: ?Sized>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_raw_fragment(writer, fragment)
    }
}

/// Serialize the given data structure as a pretty-color-printed String of JSON.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_colored_json(value: &Value) -> serde_json::Result<String> {
    let mut writer: Vec<u8> = Vec::with_capacity(128);

    write_colored_json(value, &mut writer)?;

    return Ok(String::from_utf8_lossy(&writer).to_string());
}

/// Serialize the given data structure as pretty-color-printed JSON into the IO
/// stream.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn write_colored_json<W>(
    value: &Value,
    writer: &mut W,
) -> std::result::Result<(), serde_json::Error>
where
    W: io::Write,
{
    let formatter = ColoredFormatter::new(PrettyFormatter::new());
    formatter.write_colored_json(value, writer)
}
