use ::Color;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Palette {
    pub primary: Color,
    pub secondary: Color,
    pub light_background: Color,
    pub light_background_raised: Color,
    pub light_background_text_primary: Color,
    pub light_background_divider: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            primary: super::Color::Pink500.into(),
            secondary: super::Color::PurpleA400.into(),
            light_background: super::Color::LightBackground.into(),
            light_background_raised: super::Color::LightBackgroundCard.into(),
            light_background_text_primary: super::Color::LightBackgroundTextPrimary.into(),
            light_background_divider: super::Color::LightBackgroundDivider.into(),
        }
    }
}
