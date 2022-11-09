use std::io::{Result, Write};

use crate::{color::Color, state::Materials};

/// Set terminal text color
pub fn set_text_color(mut stdout: &std::io::Stdout, r: u8, g: u8, b: u8) -> Result<()> {
    stdout.write(
        [
            "\x1b[38;2;",
            &r.to_string(),
            ";",
            &g.to_string(),
            ";",
            &b.to_string(),
            "m",
        ]
        .concat()
        .as_bytes(),
    )?;
    stdout.flush()?;
    Ok(())
}

/// Reset terminal text color
pub fn clear_text_color(mut stdout: &std::io::Stdout) -> Result<()> {
    stdout.write("\x1b[0m".as_bytes())?;
    stdout.flush()?;
    Ok(())
}

/// Trait for appending color escape codes to strings
pub trait TextColor<'a> {
    fn rgb(&'a self, r: u8, g: u8, b: u8) -> String;
    fn color(&'a self, color: Color) -> String;
}

/// Implement color trait to str
impl<'a> TextColor<'a> for &'a str {
    fn rgb(&'a self, r: u8, g: u8, b: u8) -> String {
        [
            "\x1b[38;2;",
            &r.to_string(),
            ";",
            &g.to_string(),
            ";",
            &b.to_string(),
            "m",
            *self,
            "\x1b[0m",
        ]
        .concat()
    }

    fn color(&'a self, color: Color) -> String {
        self.rgb(color.r, color.g, color.b)
    }
}

/// Implement color trait to String
impl<'a> TextColor<'a> for String {
    fn rgb(&'a self, r: u8, g: u8, b: u8) -> String {
        [
            "\x1b[38;2;",
            &r.to_string(),
            ";",
            &g.to_string(),
            ";",
            &b.to_string(),
            "m",
            self,
            "\x1b[0m",
        ]
        .concat()
    }

    fn color(&'a self, color: Color) -> String {
        self.rgb(color.r, color.g, color.b)
    }
}

/// Implement color trait to char
impl<'a> TextColor<'a> for char {
    fn rgb(&'a self, r: u8, g: u8, b: u8) -> String {
        [
            "\x1b[38;2;",
            &r.to_string(),
            ";",
            &g.to_string(),
            ";",
            &b.to_string(),
            "m",
            &self.to_string(),
            "\x1b[0m",
        ]
        .concat()
    }

    fn color(&'a self, color: Color) -> String {
        self.rgb(color.r, color.g, color.b)
    }
}

/// Implement color trait to u32
impl<'a> TextColor<'a> for u32 {
    fn rgb(&'a self, r: u8, g: u8, b: u8) -> String {
        [
            "\x1b[38;2;",
            &r.to_string(),
            ";",
            &g.to_string(),
            ";",
            &b.to_string(),
            "m",
            &self.to_string(),
            "\x1b[0m",
        ]
        .concat()
    }

    fn color(&'a self, color: Color) -> String {
        self.rgb(color.r, color.g, color.b)
    }
}

/// Implement color trait to `Materials`
impl<'a> TextColor<'a> for Materials {
    fn rgb(&'a self, r: u8, g: u8, b: u8) -> String {
        [
            "\x1b[38;2;",
            &r.to_string(),
            ";",
            &g.to_string(),
            ";",
            &b.to_string(),
            "m",
            &format!("{:?}", self),
            "\x1b[0m",
        ]
        .concat()
    }

    fn color(&'a self, color: Color) -> String {
        self.rgb(color.r, color.g, color.b)
    }
}

/// Trait for generating status bar strings
pub trait StatusBar<'a> {
    fn status_bar(&'a self, value: i32, max: i32, length: u32) -> String;
}

/// Implement color trait to &str
impl<'a> StatusBar<'a> for &'a str {
    fn status_bar(&'a self, value: i32, max: i32, length: u32) -> String {
        let percentage = value as f32 / max as f32;
        let color = match percentage {
            y if y <= 0.25 => Color::STATUS_RED,
            y if y <= 0.50 => Color::STATUS_ORANGE,
            y if y <= 0.75 => Color::STATUS_YELLOW,
            _ => Color::STATUS_GREEN,
        };
        [
            "",
            &"[".rgb(200, 125, 200),
            &self
                .repeat((percentage * length as f32) as usize)
                .color(color),
            &"]".rgb(200, 125, 200),
        ]
        .concat()
    }
}
