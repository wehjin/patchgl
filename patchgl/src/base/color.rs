#[derive(Copy, Clone, Debug)]
pub struct Color {
    a: f32,
    r: f32,
    g: f32,
    b: f32
}

impl Color {
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