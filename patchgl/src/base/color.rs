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