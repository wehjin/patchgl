#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_into_tuple() {
        let model = Id::from((33, Enabled::False));
        assert_eq!((33, Enabled::False), model.into());
    }

    #[test]
    fn default() {
        let model = Id::default();
        assert_eq!((0, Enabled::False), model.into())
    }

    #[test]
    fn enabled_upgrades_enabled_when_number_is_higher() {
        let tests = vec![(33, 32, true), (33, 33, false), (33, 34, false)];
        tests.into_iter().for_each(|(a, b, expected)| {
            let a = Id::from((a, Enabled::True));
            let b = Id::from((b, Enabled::True));
            assert_eq!(expected, a.upgrades(&b));
        });
    }

    #[test]
    fn enabled_upgrades_disabled() {
        let tests = vec![(33, 32, true), (33, 33, true), (33, 34, true)];
        tests.into_iter().for_each(|(a, b, expected)| {
            let a = Id::from((a, Enabled::True));
            let b = Id::from((b, Enabled::False));
            assert_eq!(expected, a.upgrades(&b));
        });
    }

    #[test]
    fn disabled_never_upgrades_enabled() {
        let tests = vec![(33, 32, false), (33, 33, false), (33, 34, false)];
        tests.into_iter().for_each(|(a, b, expected)| {
            let a = Id::from((a, Enabled::False));
            let b = Id::from((b, Enabled::True));
            assert_eq!(expected, a.upgrades(&b));
        });
    }

    #[test]
    fn disabled_never_upgrades_disabled() {
        let tests = vec![(33, 32, false), (33, 33, false), (33, 34, false)];
        tests.into_iter().for_each(|(a, b, expected)| {
            let a = Id::from((a, Enabled::False));
            let b = Id::from((b, Enabled::False));
            assert_eq!(expected, a.upgrades(&b));
        });
    }

    #[test]
    fn disabled_upgrades_none() {
        let a = Id::from((33, Enabled::False));
        assert_eq!(false, a.upgrades_option(&Option::None));
    }

    #[test]
    fn enabled_upgrades_none() {
        let a = Id::from((33, Enabled::True));
        assert_eq!(true, a.upgrades_option(&Option::None));
    }

    #[test]
    fn bump_enables_model_and_increments_number() {
        let model = Id::from((33, Enabled::False));
        let bumped = model.bump();
        assert_eq!((34, Enabled::True), bumped.into());
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Id {
    pub number: u64,
    pub enabled: Enabled,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Enabled {
    True,
    False,
}

impl Id {
    pub fn upgrades(&self, other: &Self) -> bool {
        match (self.enabled, other.enabled) {
            (Enabled::True, Enabled::True) => self.number > other.number,
            (Enabled::True, Enabled::False) => true,
            (Enabled::False, Enabled::True) | (Enabled::False, Enabled::False) => false,
        }
    }

    pub fn upgrades_option(&self, other: &Option<Self>) -> bool {
        match other {
            &Some(ref other) => self.upgrades(other),
            &None => match self.enabled {
                Enabled::True => true,
                Enabled::False => false,
            }
        }
    }

    pub fn bump(&self) -> Self {
        Id { number: self.number + 1, enabled: Enabled::True }
    }
}

impl Default for Id {
    fn default() -> Self {
        Id { number: 0, enabled: Enabled::False }
    }
}

impl From<(u64, Enabled)> for Id {
    fn from((number, enabled): (u64, Enabled)) -> Self {
        Id { number, enabled }
    }
}

impl Into<(u64, Enabled)> for Id {
    fn into(self) -> (u64, Enabled) {
        (self.number, self.enabled)
    }
}
