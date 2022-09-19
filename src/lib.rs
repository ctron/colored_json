//!colored_json crate to output colored serde json with ANSI terminal escape codes
//!
//!**Note for Windows 10+ users:** On Windows 10+, the application must enable ANSI support first:
//!
//!```rust
//!#[cfg(windows)]
//!let enabled = colored_json::enable_ansi_support();
//!```
//!
//!# Examples
//!
//!For everything, which implements `AsRef<str>`
//!
//!```rust
//!    # extern crate serde_json;
//!    extern crate colored_json;
//!    use colored_json::prelude::*;
//!
//!    # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    println!(
//!        "{}",
//!        r#"{
//!              "array": [
//!                "ele1",
//!                "ele2"
//!              ],
//!              "float": 3.1415926,
//!              "integer": 4398798674962568,
//!              "string": "string"
//!           }
//!        "#.to_colored_json_auto()?
//!    );
//!    # Ok(())
//!    # }
//!```
//!
//!or for serde_json::Value
//!
//!```rust
//!    # extern crate serde_json;
//!    # extern crate colored_json;
//!    use serde_json::{json, Value};
//!    use colored_json::to_colored_json_auto;
//!
//!    # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let val : Value = json!({
//!      "name": "John Doe",
//!      "age": 43,
//!      "phones": [
//!        "+44 1234567",
//!        "+44 2345678"
//!      ]
//!    });
//!    let s = to_colored_json_auto(&val)?;
//!    println!("{}", s);
//!    # Ok(())
//!    # }
//!```
//!
//!With a custom color style:
//!
//!```rust
//!    # extern crate serde_json;
//!    extern crate colored_json;
//!    use colored_json::prelude::*;
//!    use colored_json::{Color, Styler};
//!
//!    # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    println!(
//!        "{}",
//!        r#"{
//!              "array": [
//!                "ele1",
//!                "ele2"
//!              ],
//!              "float": 3.1415926,
//!              "integer": 4398798674962568,
//!              "string": "string"
//!           }
//!    "#.to_colored_json_with_styler(
//!        ColorMode::default().eval(),
//!        Styler {
//!            key: Color::Green.normal(),
//!            string_value: Color::Blue.bold(),
//!            integer_value: Color::Purple.bold(),
//!            float_value: Color::Purple.italic(),
//!            object_brackets: Color::Yellow.bold(),
//!            array_brackets: Color::Cyan.bold(),
//!            ..Default::default()
//!        })?
//!    );
//!    Ok(())
//!    # }
//!```
//!
//!
//!```rust
//!    # extern crate serde_json;
//!    # extern crate colored_json;
//!
//!    use serde_json::json;
//!
//!    use colored_json::{ColoredFormatter, CompactFormatter, Color, Styler, Style};
//!
//!    # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let f = ColoredFormatter::with_styler(
//!        CompactFormatter {},
//!        Styler {
//!            key: Color::Green.normal(),
//!            string_value: Color::Blue.bold(),
//!            ..Default::default()
//!        },
//!    );
//!
//!    println!(
//!        "{}",
//!        f.clone().to_colored_json_auto(&json!({
//!          "name": "John Doe",
//!          "age": 43,
//!          "phones": [
//!            "+44 1234567",
//!            "+44 2345678"
//!          ]
//!        }))?
//!    );
//!
//!    println!(
//!        "{}",
//!        f.to_colored_json_auto(&json!({
//!            "name":"John", "age":31, "city":"New York"
//!        }))?
//!    );
//!    # Ok(())
//!    # }
//!```

use atty::Stream;
use serde::Serialize;
use serde_json::ser::Formatter;
pub use serde_json::ser::{CompactFormatter, PrettyFormatter};
use serde_json::value::Value;
use std::io;

#[cfg(windows)]
pub use yansi::enable_ascii_colors;
pub use yansi::{Color, Style};

/// Enable ANSI support (on Windows).
///
/// On Windows, the terminal needs to be put into an "ANSI mode" so that it will render colors.
/// This is not enabled by default, but this function will enable it for you.
///
/// The function is also available on other platforms, but is a no-op in this case. So you can call
/// this function in any case.
///
/// You can also directly call the function [`yansi::enable_ansi_colors`], or use any other means
/// of enabling the virtual ANSI console in Windows. Maybe some other part of your application
/// already does that.
#[inline]
pub fn enable_ansi_support() -> Result<(), ()> {
    #[cfg(windows)]
    if !yansi::enable_ascii_colors() {
        return Err(());
    }
    Ok(())
}

#[cfg(test)]
mod test;

pub mod prelude {
    pub use crate::ColorMode;
    pub use crate::ToColoredJson;
}

/// Styler lets you define the look of the colored json output
#[derive(Clone, Copy)]
pub struct Styler {
    /// style of object brackets
    pub object_brackets: Style,
    /// style of array brackets
    pub array_brackets: Style,
    /// style of object
    pub key: Style,
    /// style of string values
    pub string_value: Style,
    /// style of integer values
    pub integer_value: Style,
    /// style of float values
    pub float_value: Style,
    /// style of bool values
    pub bool_value: Style,
    /// style of the `nil` value
    pub nil_value: Style,
    /// should the quotation get the style of the inner string/key?
    pub string_include_quotation: bool,
}

/// Default style resembling the `jq` style
impl Default for Styler {
    fn default() -> Styler {
        Styler {
            object_brackets: Style::default().bold(),
            array_brackets: Style::default().bold(),
            key: Style::default().fg(Color::Blue).bold(),
            string_value: Style::default().fg(Color::Green),
            integer_value: Style::default(),
            float_value: Style::default(),
            bool_value: Style::default(),
            nil_value: Style::default(),
            string_include_quotation: true,
        }
    }
}

/// `ColoredFormatter` decorates a `Formatter` with color defined in `Styler`
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
        ColoredFormatter {
            formatter,
            styler: Styler::default(),
            in_object_key: false,
        }
    }

    pub fn with_styler(formatter: F, styler: Styler) -> Self {
        ColoredFormatter {
            formatter,
            styler,
            in_object_key: false,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_colored_json_auto(self, value: &Value) -> serde_json::Result<String> {
        self.to_colored_json(value, ColorMode::Auto(Output::StdOut))
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_colored_json(self, value: &Value, mode: ColorMode) -> serde_json::Result<String> {
        let mut writer: Vec<u8> = Vec::with_capacity(128);

        self.write_colored_json(value, &mut writer, mode)?;

        Ok(String::from_utf8_lossy(&writer).to_string())
    }

    pub fn write_colored_json<W>(
        self,
        value: &Value,
        writer: &mut W,
        mode: ColorMode,
    ) -> Result<(), serde_json::Error>
    where
        W: io::Write,
    {
        if mode.use_color() {
            let mut serializer = serde_json::Serializer::with_formatter(writer, self);
            value.serialize(&mut serializer)
        } else {
            let mut serializer = serde_json::Serializer::with_formatter(writer, self.formatter);
            value.serialize(&mut serializer)
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

    writer.write_all(style.paint(s).to_string().as_bytes())?;
    Ok(())
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
        if self.styler.string_include_quotation {
            let style = if self.in_object_key {
                self.styler.key
            } else {
                self.styler.string_value
            };
            colored(writer, style, |w| self.formatter.begin_string(w))
        } else {
            self.formatter.begin_string(writer)
        }
    }

    fn end_string<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        if self.styler.string_include_quotation {
            let style = if self.in_object_key {
                self.styler.key
            } else {
                self.styler.string_value
            };
            colored(writer, style, |w| self.formatter.end_string(w))
        } else {
            self.formatter.end_string(writer)
        }
    }

    fn write_string_fragment<W: ?Sized>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: io::Write,
    {
        let style = if self.in_object_key {
            self.styler.key
        } else {
            self.styler.string_value
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

/// Trait to add json coloring for all `AsRef<str>` like `String` and `&str`
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
        to_colored_json(&v, ColorMode::Auto(Output::StdOut))
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
        write_colored_json_with_mode(&value, writer, ColorMode::Auto(Output::StdOut))
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
    to_colored_json(value, ColorMode::Auto(Output::StdOut))
}

pub fn to_colored_json(value: &Value, mode: ColorMode) -> serde_json::Result<String> {
    let mut writer: Vec<u8> = Vec::with_capacity(128);

    write_colored_json_with_mode(value, &mut writer, mode)?;

    Ok(String::from_utf8_lossy(&writer).to_string())
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
    write_colored_json_with_mode(value, writer, ColorMode::Auto(Output::StdOut))
}

pub fn write_colored_json_with_mode<W>(
    value: &Value,
    writer: &mut W,
    mode: ColorMode,
) -> serde_json::Result<()>
where
    W: io::Write,
{
    if mode.use_color() {
        let formatter = ColoredFormatter::new(PrettyFormatter::new());
        let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);
        value.serialize(&mut serializer)
    } else {
        let formatter = PrettyFormatter::new();
        let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);
        value.serialize(&mut serializer)
    }
}

/**
    ColorMode is a switch to enforce color mode, turn it off or auto-detect, if it should be used
**/
#[derive(Clone, Copy, PartialEq)]
pub enum ColorMode {
    On,
    Off,
    Auto(Output),
}

/// Specify the output sink, which should be used for the auto detection
#[derive(Clone, Copy, PartialEq)]
pub enum Output {
    StdOut,
    StdErr,
}

/// With `ColorMode` you can implement command line options like `--color=auto|on|off` easily.
///
/// # Example:
///
/// ```rust
/// # use colored_json::{ColorMode, Output};
///
/// let option = "--color=auto";
///
/// let color_mode = match option {
///     "--color=on" => ColorMode::Off,
///     "--color=off" => ColorMode::On,
///     _ => ColorMode::default().eval(),
/// };
///
/// assert!(match color_mode {
///     ColorMode::On | ColorMode::Off => true,
///     _ => false
/// });
/// ```
impl ColorMode {
    fn is_tty(output: Output) -> bool {
        match output {
            Output::StdOut => atty::is(Stream::Stdout),
            Output::StdErr => atty::is(Stream::Stderr),
        }
    }

    /// indicates, if the `output` is a capable of displaying colors
    pub fn should_colorize(output: Output) -> bool {
        Self::is_tty(output)
    }

    /// Returns ColorMode::On or ColorMode::Off
    ///
    /// # Example:
    ///
    /// ~~~rust
    /// # use colored_json::{ColorMode, Output};
    /// let on_off = ColorMode::default().eval();
    ///
    /// assert!(match on_off {
    ///     ColorMode::On | ColorMode::Off => true,
    ///     _ => false
    /// });
    /// ~~~
    pub fn eval(self) -> Self {
        if self.use_color() {
            ColorMode::On
        } else {
            ColorMode::Off
        }
    }

    /// Indicates if color should be used
    //
    /// # Example:
    //
    /// ```rust
    /// # use colored_json::{ColorMode, Output};
    //
    /// if ColorMode::default().use_color() {
    ///     println!("We can use color! :-)");
    /// } else {
    ///     println!("No color for you! :-(");
    /// }
    //
    /// if ColorMode::Auto(Output::StdErr).use_color() {
    ///     println!("We can use color on stderr! :-)");
    /// } else {
    ///     println!("No color for you on stderr! :-(");
    /// }
    //
    /// assert_eq!(ColorMode::On.use_color(), true);
    /// assert_eq!(ColorMode::Off.use_color(), false);
    /// ```
    pub fn use_color(self) -> bool {
        match self {
            ColorMode::On => true,
            ColorMode::Off => false,
            ColorMode::Auto(output) => Self::should_colorize(output),
        }
    }
}

impl Default for ColorMode {
    /// returns `ColorMode::Auto(Output::StdOut)`
    fn default() -> Self {
        ColorMode::Auto(Output::StdOut)
    }
}
