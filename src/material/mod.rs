use ::Color;

#[allow(dead_code)]
pub enum MaterialColor {
    LightBackground,
    LightBackgroundCard,
    LightBackgroundTextPrimary,
    LightBackgroundTextSecondary,
    LightBackgroundTextDisabled,
    LightBackgroundDivider,
    PurpleA100,
    PurpleA200,
    PurpleA400,
    PurpleA700,
    Pink500,
}

impl Into<Color> for MaterialColor {
    fn into(self) -> Color {
        match self {
            MaterialColor::LightBackground => Color::from_hexrgb(0xfa, 0xfa, 0xfa),
            MaterialColor::LightBackgroundCard => Color::white(),
            MaterialColor::LightBackgroundTextPrimary => Color::custom_black(0.87),
            MaterialColor::LightBackgroundTextSecondary => Color::custom_black(0.54),
            MaterialColor::LightBackgroundTextDisabled => Color::custom_black(0.38),
            MaterialColor::LightBackgroundDivider => Color::custom_black(0.12),
            MaterialColor::PurpleA100 => Color::from_hexrgb(0xea, 0x80, 0xfc),
            MaterialColor::PurpleA200 => Color::from_hexrgb(0xe0, 0x40, 0xfb),
            MaterialColor::PurpleA400 => Color::from_hexrgb(0xd5, 0x00, 0xf9),
            MaterialColor::PurpleA700 => Color::from_hexrgb(0xaa, 0x00, 0xff),
            MaterialColor::Pink500 => Color::from_hexrgb(0xe9, 0x1e, 0x64),
        }
    }
}



