#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Length {
    Display4Text,
    Display3Text,
    Display2Text,
    Display1Text,
    HeadlineText,
    TitleText,
    SubheadingText,
    Body2Text,
    Body1Text,
    CaptionText,
    ButtonText,
}

use flood;

impl Into<flood::Length> for Length {
    fn into(self) -> flood::Length {
        match self {
            Length::Display4Text => flood::Length::Pixels(112.0),
            Length::Display3Text => flood::Length::Pixels(56.0),
            Length::Display2Text => flood::Length::Pixels(45.0),
            Length::Display1Text => flood::Length::Pixels(34.0),
            Length::HeadlineText => flood::Length::Pixels(24.0),
            Length::TitleText => flood::Length::Pixels(20.0),
            Length::SubheadingText => flood::Length::Pixels(16.0),
            Length::Body2Text => flood::Length::Pixels(14.0),
            Length::Body1Text => flood::Length::Pixels(14.0),
            Length::CaptionText => flood::Length::Pixels(12.0),
            Length::ButtonText => flood::Length::Pixels(140.0),
        }
    }
}