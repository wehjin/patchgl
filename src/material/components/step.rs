use ::flood::*;
use ::material;

#[derive(Clone, PartialEq, Debug)]
pub struct Step<'a> {
    pub id: Vec<u64>,
    pub label: &'a str,
    pub index: u32,
    pub condition: StepCondition,
    pub palette: &'a material::Palette,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StepCondition {
    Active,
    Inactive,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StepMsg {}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct StepMdl {}

impl<'a, MsgT> Into<Flood<MsgT>> for Step<'a> where MsgT: Clone {
    fn into(self) -> Flood<MsgT> {
        let digit_on_badge = {
            let digit = Flood::Text(format!("{}", self.index + 1), self.palette.dark_background_text_primary, Placement::Center);
            let badge = {
                let color = match self.condition {
                    StepCondition::Active => self.palette.primary,
                    StepCondition::Inactive => self.palette.light_background_disabled,
                };
                Flood::Color(color)
            };
            digit + (Stratum::JustBelow, badge)
        };
        let gap = Flood::Color(self.palette.transparent);
        let label = {
            let color = match self.condition {
                StepCondition::Active => self.palette.light_background_text_primary,
                StepCondition::Inactive => self.palette.light_background_disabled,
            };
            Flood::Text(self.label.to_owned(), color, Placement::Start)
        };
        label + (Position::Left(Length::Cross / 3), gap) + (Position::Left(Length::Cross), digit_on_badge)
    }
}
