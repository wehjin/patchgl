use ::flood::*;
use ::material;

#[derive(Clone, Debug)]
pub struct Stepper<'a, MsgT> where MsgT: Clone {
    pub palette: &'a material::Palette,
    pub id: Vec<u64>,
    pub active_index: usize,
    pub active_content: Flood<MsgT>,
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
    Completed,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StepperMsg {}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct StepperMdl {}

impl<'a, MsgT> Into<Flood<MsgT>> for Stepper<'a, MsgT> where MsgT: Clone {
    fn into(self) -> Flood<MsgT> {
        use self::bar::Bar;
        let palette = self.palette;
        let active_index = self.active_index;
        let steps = self.steps;
        let bar: Flood<MsgT> = Bar { palette, active_index, steps }.into();

        let raised_details = self.active_content
            + (Stratum::JustBelow, Flood::Color(palette.light_background_raised))
            + Padding::Behind(Length::CardApproach);

        raised_details
            + (Position::Top(Length::Spacing / 2), Flood::Color(palette.transparent))
            + (Position::Top(Length::Full * 0.10), bar)
    }
}

mod bar {
    use ::material::Palette;
    use ::flood::*;
    use super::badge::Badge;
    use super::label::Label;
    use super::spacer::Spacer;
    use super::{StepCondition, Step};

    #[derive(Clone, PartialEq, Debug)]
    pub struct Bar<'a> {
        pub palette: &'a Palette,
        pub active_index: usize,
        pub steps: Vec<Step<'a>>,
    }

    impl<'a, MsgT> Into<Flood<MsgT>> for Bar<'a> where MsgT: Clone {
        fn into(self) -> Flood<MsgT> {
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
                    let condition = if index < active_index {
                        StepCondition::Completed
                    } else if index == active_index {
                        StepCondition::Active
                    } else {
                        StepCondition::Inactive
                    };
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
            Flood::Color(self.palette.light_background_text_disabled) + Padding::Dual(Length::Cross / 3, Length::Full * (0.5 * 23.0 / 24.0))
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
                StepCondition::Active | StepCondition::Completed => self.palette.light_background_text_primary,
                StepCondition::Inactive => self.palette.light_background_text_disabled,
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
            let text = match self.condition {
                StepCondition::Completed => "✓".to_owned(),
                StepCondition::Active | StepCondition::Inactive => format!("{}", self.digit),
            };
            let digit = Flood::Text(text, self.palette.dark_background_text_primary, Placement::Center)
                + Padding::Uniform(Length::Full * 0.15);

            let badge = {
                let color = match self.condition {
                    StepCondition::Active | StepCondition::Completed => self.palette.primary,
                    StepCondition::Inactive => self.palette.light_background_text_disabled,
                };
                Flood::Color(color)
            };
            digit + (Stratum::JustBelow, badge)
        }
    }
}
