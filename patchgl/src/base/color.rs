pub enum WebColor {
    DeepPink
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    a: f32,
    r: f32,
    g: f32,
    b: f32
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
    pub fn new(a: f32, r: f32, g: f32, b: f32) -> Self {
        Color { a: a, r: r, g: g, b: b }
    }
    pub fn from_hexrgb(hex_r: u8, hex_g: u8, hex_b: u8) -> Self {
        Color { a: 1.0, r: hex_r as f32 / 255.0, g: hex_g as f32 / 255.0, b: hex_b as f32 / 255.0 }
    }

    pub fn from_web(web_color: WebColor) -> Self {
        match web_color {
            WebColor::DeepPink => Color::from_hexrgb(0xff, 0x14, 0x93)
        }
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