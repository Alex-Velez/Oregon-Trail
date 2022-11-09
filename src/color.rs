#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Return status colors based on percentage given
    pub fn percentage(value: i32, max: i32) -> Color {
        let percentage = value as f32 / max as f32;
        match percentage {
            y if y <= 0.25 => Color::STATUS_RED,
            y if y <= 0.50 => Color::STATUS_ORANGE,
            y if y <= 0.75 => Color::STATUS_YELLOW,
            _ => Color::STATUS_GREEN,
        }
    }

    /// Return status colors based on percentage given
    pub fn percentage_flip(value: i32, max: i32) -> Color {
        let percentage = value as f32 / max as f32;
        match percentage {
            y if y <= 0.25 => Color::STATUS_GREEN,
            y if y <= 0.50 => Color::STATUS_YELLOW,
            y if y <= 0.75 => Color::STATUS_ORANGE,
            _ => Color::STATUS_RED,
        }
    }
}

/// Oregon color constants
impl Color {
    pub const STATUS_RED: Color = Color::new(200, 50, 100);
    pub const STATUS_ORANGE: Color = Color::new(200, 100, 50);
    pub const STATUS_YELLOW: Color = Color::new(200, 150, 50);
    pub const STATUS_GREEN: Color = Color::new(150, 250, 0);
    pub const BAR_YELLOW: Color = Color::new(255, 215, 0);
}
