pub mod argb;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum WebColor {
    Blue,
    DeepPink,
    Grey,
    Green,
    Red,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum X11Color {
    Lavender,
    Thistle,
    Plum,
    MediumPurple,
    Indigo,
}

impl WebColor {
    pub fn from_name(name: &str) -> Self {
        match name {
            "blue" => WebColor::Blue,
            "green" => WebColor::Green,
            "grey" => WebColor::Grey,
            "deeppink" => WebColor::DeepPink,
            "red" => WebColor::Red,
            _ => WebColor::DeepPink,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Default for Color {
    fn default() -> Self {
        WebColor::DeepPink.into()
    }
}

impl From<WebColor> for Color {
    fn from(web_color: WebColor) -> Self {
        match web_color {
            WebColor::Blue => Color::blue(),
            WebColor::DeepPink => Color::from_hexrgb(0xff, 0x14, 0x93),
            WebColor::Green => Color::green(),
            WebColor::Grey => Color::grey(),
            WebColor::Red => Color::red(),
        }
    }
}

impl From<X11Color> for Color {
    fn from(x11_color: X11Color) -> Self {
        match x11_color {
            X11Color::Lavender => Color::from_hexrgb(0xe6, 0xe6, 0xfa),
            X11Color::Thistle => Color::from_hexrgb(0xd8, 0xbf, 0xd8),
            X11Color::Plum => Color::from_hexrgb(0xdd, 0xa0, 0xdd),
            X11Color::MediumPurple => Color::from_hexrgb(0x93, 0x70, 0xd9),
            X11Color::Indigo => Color::from_hexrgb(0x4b, 0x00, 0x82),
        }
    }
}

impl Color {
    pub fn to_gl(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn white() -> Self { Color { a: 1.0, r: 1.0, g: 1.0, b: 1.0 } }
    pub fn grey() -> Self { Color { a: 1.0, r: 0.5, g: 0.5, b: 0.5 } }
    pub fn black() -> Self { Color { a: 1.0, r: 0.0, g: 0.0, b: 0.0 } }
    pub fn red() -> Self { Color { a: 1.0, r: 1.0, g: 0.0, b: 0.0 } }
    pub fn green() -> Self { Color { a: 1.0, r: 0.0, g: 1.0, b: 0.0 } }
    pub fn blue() -> Self { Color { a: 1.0, r: 0.0, g: 0.0, b: 1.0 } }
    pub fn custom_black(fraction: f32) -> Self { Color::custom_white((1.0 - fraction).max(0.0)) }
    pub fn custom_white(fraction: f32) -> Self {
        let fraction = fraction.min(1.0);
        Color { a: 1.0, r: fraction, g: fraction, b: fraction }
    }
    pub fn from_hexrgb(hex_r: u8, hex_g: u8, hex_b: u8) -> Self {
        Color { a: 1.0, r: hex_r as f32 / 255.0, g: hex_g as f32 / 255.0, b: hex_b as f32 / 255.0 }
    }
    pub fn new(a: f32, r: f32, g: f32, b: f32) -> Self {
        Color { a, r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_constructor_takes_argb_parameters() {
        let color = Color::new(1.0, 0.8, 0.6, 0.4);
        assert_eq!(1.0, color.a)
    }
}