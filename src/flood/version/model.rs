#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_into_tuple() {
        let model = Counter::from((33, Enabled::False));
        assert_eq!((33, Enabled::False), model.into());
    }

    #[test]
    fn default() {
        let model = Counter::default();
        assert_eq!((0, Enabled::False), model.into())
    }

    #[test]
    fn enabled_upgrades_enabled_when_number_is_higher() {
        let tests = vec![(33, 32, true), (33, 33, false), (33, 34, false)];
        tests.into_iter().for_each(|(a, b, expected)| {
            let a = Counter::from((a, Enabled::True));
            let b = Counter::from((b, Enabled::True));
            assert_eq!(expected, a.upgrades(&b));
        });
    }

    #[test]
    fn enabled_upgrades_disabled() {
        let tests = vec![(33, 32, true), (33, 33, true), (33, 34, true)];
        tests.into_iter().for_each(|(a, b, expected)| {
            let a = Counter::from((a, Enabled::True));
            let b = Counter::from((b, Enabled::False));
            assert_eq!(expected, a.upgrades(&b));
        });
    }

    #[test]
    fn disabled_never_upgrades_enabled() {
        let tests = vec![(33, 32, false), (33, 33, false), (33, 34, false)];
        tests.into_iter().for_each(|(a, b, expected)| {
            let a = Counter::from((a, Enabled::False));
            let b = Counter::from((b, Enabled::True));
            assert_eq!(expected, a.upgrades(&b));
        });
    }

    #[test]
    fn disabled_never_upgrades_disabled() {
        let tests = vec![(33, 32, false), (33, 33, false), (33, 34, false)];
        tests.into_iter().for_each(|(a, b, expected)| {
            let a = Counter::from((a, Enabled::False));
            let b = Counter::from((b, Enabled::False));
            assert_eq!(expected, a.upgrades(&b));
        });
    }

    #[test]
    fn disabled_upgrades_none() {
        let a = Counter::from((33, Enabled::False));
        assert_eq!(false, a.upgrades_option(&Option::None));
    }

    #[test]
    fn enabled_upgrades_none() {
        let a = Counter::from((33, Enabled::True));
        assert_eq!(true, a.upgrades_option(&Option::None));
    }

    #[test]
    fn bump_enables_model_and_increments_number() {
        let counter = Counter::from((33, Enabled::False));
        let mut bumped = counter.to_owned();
        bumped.bump();
        assert_eq!((34, Enabled::True), bumped.into());
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Counter {
    pub number: u64,
    pub enabled: Enabled,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Enabled {
    True,
    False,
}

impl Counter {
    pub fn enabled() -> Self {
        Counter { number: 1, enabled: Enabled::True }
    }
    pub fn enabled_after_bump() -> Self { Counter { number: 0, enabled: Enabled::False } }

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

    pub fn bump(&mut self) {
        self.number += 1;
        self.enabled = Enabled::True;
    }
}

impl Default for Counter {
    fn default() -> Self {
        Counter { number: 0, enabled: Enabled::False }
    }
}

impl From<(u64, Enabled)> for Counter {
    fn from((number, enabled): (u64, Enabled)) -> Self {
        Counter { number, enabled }
    }
}

impl Into<(u64, Enabled)> for Counter {
    fn into(self) -> (u64, Enabled) {
        (self.number, self.enabled)
    }
}
