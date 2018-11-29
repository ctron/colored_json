/*!
colored_json crate to output colored serde json with ANSI terminal escape codes

# Examples

For everything, which implements AsRef<str>

```rust
    # extern crate serde_json;
    extern crate colored_json;
    use colored_json::prelude::*;

    # fn main() -> ::std::result::Result<(), Box<::std::error::Error>> {
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
        "#.to_colored_json_auto()?
    );
    # Ok(())
    # }
```

or for serde_json::Value

```rust
    # extern crate serde_json;
    # extern crate colored_json;
    use serde_json::{json, Value};
    use colored_json::to_colored_json_auto;

    # fn main() -> ::std::result::Result<(), Box<::std::error::Error>> {
    let val : Value = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });
    let s = to_colored_json_auto(&val)?;
    println!("{}", s);
    # Ok(())
    # }
```

With a custom color style:

```rust
    # extern crate serde_json;
    extern crate colored_json;
    use colored_json::prelude::*;
    use colored_json::{Color, Styler};

    # fn main() -> ::std::result::Result<(), Box<::std::error::Error>> {
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
    "#.to_colored_json_with_styler(
        ColorMode::Auto,
        Styler {
            key: Color::Green.normal(),
            string_value: Color::Blue.bold(),
            integer_value: Color::Purple.bold(),
            float_value: Color::Purple.italic(),
            object_brackets: Color::Yellow.bold(),
            array_brackets: Color::Cyan.bold(),
            ..Default::default()
        })?
    );
    Ok(())
    # }
```


```rust
    # extern crate serde_json;
    # extern crate colored_json;

    use serde_json::json;
    use serde_json::ser::CompactFormatter;

    use colored_json::{ColoredFormatter, Color, Styler, Style};

    # fn main() -> ::std::result::Result<(), Box<::std::error::Error>> {
    let f = ColoredFormatter::with_styler(
        CompactFormatter {},
        Styler {
            key: Color::Green.normal(),
            string_value: Color::Blue.bold(),
            ..Default::default()
        },
    );

    println!(
        "{}",
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
    # Ok(())
    # }
```

!*/

extern crate ansi_term;
extern crate serde;
extern crate serde_json;

#[cfg(unix)]
extern crate libc;

pub use ansi_term::Colour;
pub use ansi_term::Colour as Color;
pub use ansi_term::Style;
use serde::Serialize;
use serde_json::ser::{Formatter, PrettyFormatter};
use serde_json::value::Value;

use std::io;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use ColorMode;
    pub use ToColoredJson;
}

#[derive(Clone)]
pub struct Styler {
    pub object_brackets: Style,
    pub array_brackets: Style,
    pub key: Style,
    pub string_value: Style,
    pub integer_value: Style,
    pub float_value: Style,
    pub bool_value: Style,
    pub nil_value: Style,
}

impl Default for Styler {
    fn default() -> Styler {
        Styler {
            object_brackets: Style::new().bold(),
            array_brackets: Style::new().bold(),
            key: Style::new().fg(Color::Blue).bold(),
            string_value: Style::new().fg(Color::Green),
            integer_value: Style::new(),
            float_value: Style::new(),
            bool_value: Style::new(),
            nil_value: Style::new(),
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
    in_object_key: bool,
}

impl<F> ColoredFormatter<F>
where
    F: Formatter,
{
    pub fn new(formatter: F) -> Self {
        return ColoredFormatter {
            formatter,
            styler: Styler::default(),
            in_object_key: false,
        };
    }

    pub fn with_styler(formatter: F, styler: Styler) -> Self {
        return ColoredFormatter {
            formatter,
            styler: styler,
            in_object_key: false,
        };
    }

    pub fn to_colored_json_auto(self, value: &Value) -> serde_json::Result<String> {
        self.to_colored_json(value, ColorMode::Auto)
    }

    pub fn to_colored_json(self, value: &Value, mode: ColorMode) -> serde_json::Result<String> {
        let mut writer: Vec<u8> = Vec::with_capacity(128);

        self.write_colored_json(value, &mut writer, mode)?;

        return Ok(String::from_utf8_lossy(&writer).to_string());
    }

    pub fn write_colored_json<W>(
        self,
        value: &Value,
        writer: &mut W,
        mode: ColorMode,
    ) -> std::result::Result<(), serde_json::Error>
    where
        W: io::Write,
    {
        match mode.use_color() {
            true => {
                let mut serializer = serde_json::Serializer::with_formatter(writer, self);

                return value.serialize(&mut serializer);
            }
            false => {
                let mut serializer = serde_json::Serializer::with_formatter(writer, self.formatter);
                return value.serialize(&mut serializer);
            }
        }
    }
}

fn colored<W: ?Sized, H>(writer: &mut W, style: Style, mut handler: H) -> io::Result<()>
where
    W: io::Write,
    H: FnMut(&mut Vec<u8>) -> io::Result<()>,
{
    let mut w: Vec<u8> = Vec::with_capacity(128);
    handler(&mut w)?;
    let s = String::from_utf8_lossy(&w);

    Ok(writer.write_all(style.paint(s).to_string().as_bytes())?)
}

impl<F> Formatter for ColoredFormatter<F>
where
    F: Formatter,
{
    fn write_null<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.nil_value, |w| {
            self.formatter.write_null(w)
        })
    }

    fn write_bool<W: ?Sized>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.bool_value, |w| {
            self.formatter.write_bool(w, value)
        })
    }

    fn write_i8<W: ?Sized>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_i8(w, value)
        })
    }

    fn write_i16<W: ?Sized>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_i16(w, value)
        })
    }

    fn write_i32<W: ?Sized>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_i32(w, value)
        })
    }

    fn write_i64<W: ?Sized>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_i64(w, value)
        })
    }

    fn write_u8<W: ?Sized>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_u8(w, value)
        })
    }

    fn write_u16<W: ?Sized>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_u16(w, value)
        })
    }

    fn write_u32<W: ?Sized>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_u32(w, value)
        })
    }

    fn write_u64<W: ?Sized>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_u64(w, value)
        })
    }

    fn write_f32<W: ?Sized>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.float_value, |w| {
            self.formatter.write_f32(w, value)
        })
    }

    fn write_f64<W: ?Sized>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.float_value, |w| {
            self.formatter.write_f64(w, value)
        })
    }

    fn write_number_str<W: ?Sized>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.integer_value, |w| {
            self.formatter.write_number_str(w, value)
        })
        //        self.formatter.write_number_str(writer, value)
    }

    fn begin_string<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let style = match self.in_object_key {
            true => self.styler.key,
            false => self.styler.string_value,
        };
        colored(writer, style, |w| self.formatter.begin_string(w))
    }

    fn end_string<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let style = match self.in_object_key {
            true => self.styler.key,
            false => self.styler.string_value,
        };
        colored(writer, style, |w| self.formatter.end_string(w))
    }

    fn write_string_fragment<W: ?Sized>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: io::Write,
    {
        let style = match self.in_object_key {
            true => self.styler.key,
            false => self.styler.string_value,
        };
        colored(writer, style, |w| {
            self.formatter.write_string_fragment(w, fragment)
        })
    }

    fn begin_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.array_brackets, |w| {
            self.formatter.begin_array(w)
        })
    }

    fn end_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.array_brackets, |w| {
            self.formatter.end_array(w)
        })
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
        colored(writer, self.styler.object_brackets, |w| {
            self.formatter.begin_object(w)
        })
    }

    fn end_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        colored(writer, self.styler.object_brackets, |w| {
            self.formatter.end_object(w)
        })
    }

    fn begin_object_key<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        self.in_object_key = true;
        self.formatter.begin_object_key(writer, first)
    }

    fn end_object_key<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.in_object_key = false;
        self.formatter.end_object_key(writer)?;
        Ok(())
    }

    fn begin_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.in_object_key = false;
        self.formatter.begin_object_value(writer)
    }

    fn end_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.in_object_key = false;
        self.formatter.end_object_value(writer)?;
        Ok(())
    }

    fn write_raw_fragment<W: ?Sized>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: io::Write,
    {
        self.formatter.write_raw_fragment(writer, fragment)
    }
}

pub trait ToColoredJson {
    fn to_colored_json_auto(&self) -> serde_json::Result<String>;
    fn to_colored_json(&self, mode: ColorMode) -> serde_json::Result<String>;
    fn to_colored_json_with_styler(
        &self,
        mode: ColorMode,
        styler: Styler,
    ) -> serde_json::Result<String>;
    fn write_colored_json<W>(&self, writer: &mut W) -> serde_json::Result<()>
    where
        W: io::Write;
    fn write_colored_json_with_mode<W>(
        &self,
        writer: &mut W,
        mode: ColorMode,
    ) -> serde_json::Result<()>
    where
        W: io::Write;
    fn write_colored_json_with_styler<W>(
        &self,
        writer: &mut W,
        mode: ColorMode,
        styler: Styler,
    ) -> serde_json::Result<()>
    where
        W: io::Write;
}

impl<S> ToColoredJson for S
where
    S: ?Sized + AsRef<str>,
{
    /// Serialize the given data structure as a pretty-color-printed String of JSON.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    fn to_colored_json_auto(&self) -> serde_json::Result<String> {
        let v: Value = serde_json::from_str(self.as_ref())?;
        to_colored_json(&v, ColorMode::Auto)
    }

    /// Serialize the given data structure as a pretty-color-printed String of JSON.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    fn to_colored_json(&self, mode: ColorMode) -> serde_json::Result<String> {
        let v: Value = serde_json::from_str(self.as_ref())?;
        to_colored_json(&v, mode)
    }

    /// Serialize the given data structure as a pretty-color-printed String of JSON.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    fn to_colored_json_with_styler(
        &self,
        mode: ColorMode,
        styler: Styler,
    ) -> serde_json::Result<String> {
        let f = ColoredFormatter::with_styler(PrettyFormatter::new(), styler);
        let v: Value = serde_json::from_str(self.as_ref())?;
        f.to_colored_json(&v, mode)
    }

    /// Serialize the given data structure as pretty-color-printed JSON into the IO
    /// stream.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    fn write_colored_json<W>(&self, writer: &mut W) -> serde_json::Result<()>
    where
        W: io::Write,
    {
        let value: Value = serde_json::from_str(self.as_ref())?;
        write_colored_json_with_mode(&value, writer, ColorMode::Auto)
    }

    fn write_colored_json_with_mode<W>(
        &self,
        writer: &mut W,
        mode: ColorMode,
    ) -> serde_json::Result<()>
    where
        W: io::Write,
    {
        let value: Value = serde_json::from_str(self.as_ref())?;
        write_colored_json_with_mode(&value, writer, mode)
    }

    fn write_colored_json_with_styler<W>(
        &self,
        writer: &mut W,
        mode: ColorMode,
        styler: Styler,
    ) -> serde_json::Result<()>
    where
        W: io::Write,
    {
        let value: Value = serde_json::from_str(self.as_ref())?;
        let f = ColoredFormatter::with_styler(PrettyFormatter::new(), styler);
        f.write_colored_json(&value, writer, mode)
    }
}

/// Serialize the given data structure as a pretty-color-printed String of JSON.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_colored_json_auto(value: &Value) -> serde_json::Result<String> {
    to_colored_json(value, ColorMode::Auto)
}

pub fn to_colored_json(value: &Value, mode: ColorMode) -> serde_json::Result<String> {
    let mut writer: Vec<u8> = Vec::with_capacity(128);

    write_colored_json_with_mode(value, &mut writer, mode)?;

    return Ok(String::from_utf8_lossy(&writer).to_string());
}

/// Serialize the given data structure as pretty-color-printed JSON into the IO
/// stream.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn write_colored_json<W>(value: &Value, writer: &mut W) -> serde_json::Result<()>
where
    W: io::Write,
{
    write_colored_json_with_mode(value, writer, ColorMode::Auto)
}

pub fn write_colored_json_with_mode<W>(
    value: &Value,
    writer: &mut W,
    mode: ColorMode,
) -> serde_json::Result<()>
where
    W: io::Write,
{
    match mode.use_color() {
        true => {
            let formatter = ColoredFormatter::new(PrettyFormatter::new());
            let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);
            value.serialize(&mut serializer)
        }
        false => {
            let formatter = PrettyFormatter::new();
            let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);
            value.serialize(&mut serializer)
        }
    }
}

#[derive(Clone)]
pub enum ColorMode {
    On,
    Off,
    Auto,
    AutoErr,
}

pub enum Output {
    StdOut,
    StdErr,
}

impl ColorMode {
    #[cfg(unix)]
    fn is_tty(output: Output) -> bool {
        use libc;

        let fd = match output {
            Output::StdOut => libc::STDOUT_FILENO,
            Output::StdErr => libc::STDERR_FILENO,
        } as i32;

        unsafe { libc::isatty(fd) != 0 }
    }

    #[cfg(not(unix))]
    fn is_tty(output: Output) -> bool {
        false
    }

    pub fn should_colorize(output: Output) -> bool {
        Self::is_tty(output)
    }

    pub fn use_color(&self) -> bool {
        match self {
            ColorMode::On => true,
            ColorMode::Off => false,
            ColorMode::Auto => Self::should_colorize(Output::StdOut),
            ColorMode::AutoErr => Self::should_colorize(Output::StdErr),
        }
    }
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Auto
    }
}
