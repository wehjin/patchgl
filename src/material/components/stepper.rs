use ::flood::*;
use ::material;

#[derive(Clone, PartialEq, Debug)]
pub struct Stepper<'a> {
    pub palette: &'a material::Palette,
    pub id: Vec<u64>,
    pub active_index: usize,
    pub steps: Vec<Step<'a>>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Step<'a> {
    pub label: &'a str,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StepCondition {
    Active,
    Inactive,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StepperMsg {}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct StepperMdl {}

impl<'a, MsgT> Into<Flood<MsgT>> for Stepper<'a> where MsgT: Clone {
    fn into(self) -> Flood<MsgT> {
        use self::badge::Badge;
        use self::label::Label;
        use self::spacer::Spacer;

        let palette = self.palette;
        let flood = if self.steps.is_empty() {
            let spacer: Flood<MsgT> = Spacer { palette }.into();
            spacer
        } else {
            let active_index = self.active_index;
            let last_index = self.steps.len() - 1;
            let enumerated_steps = self.steps.into_iter().enumerate().collect::<Vec<_>>();
            enumerated_steps.into_iter().fold(Flood::Color(palette.transparent), |flood, (index, step)| {
                let text = step.label;
                let condition = if index == active_index { StepCondition::Active } else { StepCondition::Inactive };
                let badge: Flood<MsgT> = Badge { palette, digit: index as u32 + 1, condition }.into();
                let gap = Flood::Color(palette.transparent);
                let label: Flood<MsgT> = Label { palette, text, condition }.into();
                let flood = if index < last_index {
                    let spacer: Flood<MsgT> = Spacer { palette }.into();
                    let segment = spacer
                        + (Position::Left(Length::Text(text.to_owned())), label)
                        + (Position::Left(Length::Cross / 3), gap)
                        + (Position::Left(Length::Cross), badge);
                    flood + (Position::Right(Length::Full / (index + 1)), segment)
                } else {
                    flood
                        + (Position::Right(Length::Cross), badge)
                        + (Position::Right(Length::Cross / 3), gap)
                        + (Position::Right(Length::Text(text.to_owned())), label)
                };
                flood
            })
        };
        flood + Padding::Uniform(Length::Cross * 0.3)
            + (Stratum::JustBelow, Flood::Color(palette.light_background_raised))
            + Padding::Behind(Length::CardApproach)
    }
}

mod spacer {
    use ::material::Palette;
    use ::flood::*;

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Spacer<'a> {
        pub palette: &'a Palette,
    }

    impl<'a, MsgT> Into<Flood<MsgT>> for Spacer<'a> where MsgT: Clone {
        fn into(self) -> Flood<MsgT> {
            Flood::Color(self.palette.light_background_disabled) + Padding::Dual(Length::Cross / 3, Length::Full * (0.5 * 23.0 / 24.0))
        }
    }
}

mod label {
    use ::material::Palette;
    use ::flood::*;
    use super::StepCondition;

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Label<'a> {
        pub palette: &'a Palette,
        pub text: &'a str,
        pub condition: StepCondition,
    }

    impl<'a, MsgT> Into<Flood<MsgT>> for Label<'a> where MsgT: Clone {
        fn into(self) -> Flood<MsgT> {
            let color = match self.condition {
                StepCondition::Active => self.palette.light_background_text_primary,
                StepCondition::Inactive => self.palette.light_background_disabled,
            };
            Flood::Text(self.text.to_owned(), color, Placement::Start)
        }
    }
}


mod badge {
    use ::material::Palette;
    use ::flood::*;
    use super::StepCondition;

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Badge<'a> {
        pub palette: &'a Palette,
        pub digit: u32,
        pub condition: StepCondition,
    }

    impl<'a, MsgT> Into<Flood<MsgT>> for Badge<'a> where MsgT: Clone {
        fn into(self) -> Flood<MsgT> {
            let digit = Flood::Text(format!("{}", self.digit), self.palette.dark_background_text_primary, Placement::Center)
                + Padding::Uniform(Length::Full * 0.15);

            let badge = {
                let color = match self.condition {
                    StepCondition::Active => self.palette.primary,
                    StepCondition::Inactive => self.palette.light_background_disabled,
                };
                Flood::Color(color)
            };
            digit + (Stratum::JustBelow, badge)
        }
    }
}
