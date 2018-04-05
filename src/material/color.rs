use ::Color as ArgbColor;

#[allow(dead_code)]
pub enum Color {
    LightBackground,
    LightBackgroundCard,
    LightBackgroundTextPrimary,
    LightBackgroundTextSecondary,
    LightBackgroundTextDisabled,
    LightBackgroundDivider,
    Pink500,
    PinkA100,
    PinkA200,
    PinkA400,
    PinkA700,
    PurpleA100,
    PurpleA200,
    PurpleA400,
    PurpleA700,
}

impl Into<ArgbColor> for Color {
    fn into(self) -> ArgbColor {
        match self {
            Color::LightBackground => ArgbColor::from_hexrgb(0xfa, 0xfa, 0xfa),
            Color::LightBackgroundCard => ArgbColor::white(),
            Color::LightBackgroundTextPrimary => ArgbColor::custom_black(0.87),
            Color::LightBackgroundTextSecondary => ArgbColor::custom_black(0.54),
            Color::LightBackgroundTextDisabled => ArgbColor::custom_black(0.38),
            Color::LightBackgroundDivider => ArgbColor::custom_black(0.12),
            Color::Pink500 => ArgbColor::from_hexrgb(0xe9, 0x1e, 0x64),
            Color::PinkA100 => ArgbColor::from_hexrgb(0xFF, 0x80, 0xAB),
            Color::PinkA200 => ArgbColor::from_hexrgb(0xFF, 0x40, 0x81),
            Color::PinkA400 => ArgbColor::from_hexrgb(0xF5, 0x00, 0x57),
            Color::PinkA700 => ArgbColor::from_hexrgb(0xC5, 0x11, 0x62),
            Color::PurpleA100 => ArgbColor::from_hexrgb(0xea, 0x80, 0xfc),
            Color::PurpleA200 => ArgbColor::from_hexrgb(0xe0, 0x40, 0xfb),
            Color::PurpleA400 => ArgbColor::from_hexrgb(0xd5, 0x00, 0xf9),
            Color::PurpleA700 => ArgbColor::from_hexrgb(0xaa, 0x00, 0xff),
        }
    }
}
