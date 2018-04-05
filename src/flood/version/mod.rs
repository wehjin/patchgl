pub use self::model::Counter;
use std::fmt;

mod model;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let version = Version::from(33);
        assert_eq!(33, version.value);
    }

    #[test]
    fn into_tuple() {
        let version = Version::from(33);
        let into_tuple = version.into();
        assert_eq!((33, Counter::default()), into_tuple);
    }

    #[test]
    fn from_tuple() {
        let from_tuple = Version::from((33, Counter::default()));
        assert_eq!(Version::from(33), from_tuple);
    }

    #[test]
    fn bumped_upgrades_original() {
        let a = Version::from(33);
        let b = a.bump(26);
        assert!(b.upgrades(&a));
    }

    #[test]
    fn original_does_not_upgrade_original() {
        let a = Version::from(33);
        let b = Version::from(33);
        assert!(!a.upgrades(&b));
    }

    #[test]
    fn default_does_not_upgrade_none() {
        let a = Version::from(33);
        assert!(!a.upgrades_option(&Option::None));
    }

    #[test]
    fn bumped_upgrades_none() {
        let a = Version::from(33).bump(13);
        assert!(a.upgrades_option(&Option::None));
    }
}

pub struct Version<T> {
    pub value: T,
    pub counter: Counter,
}

impl<T> Version<T> {
    pub fn upgrades(&self, other: &Self) -> bool {
        self.counter.upgrades(&other.counter)
    }

    pub fn upgrades_option(&self, other: &Option<Self>) -> bool {
        match other {
            &Option::Some(ref other) => self.upgrades(other),
            &Option::None => self.counter.upgrades_option(&Option::None),
        }
    }

    pub fn bump(&self, value: T) -> Self {
        Version { value, counter: self.counter.bump() }
    }
}

impl<T> Clone for Version<T> where T: Clone {
    fn clone(&self) -> Self {
        Version { value: self.value.clone(), counter: self.counter.clone() }
    }
}

impl<T> From<(T, Counter)> for Version<T> {
    fn from((value, counter): (T, Counter)) -> Self {
        Version { value, counter }
    }
}

impl<T> From<T> for Version<T> {
    fn from(value: T) -> Self {
        Version { counter: Counter::default(), value }
    }
}

impl<T> Into<(T, Counter)> for Version<T> {
    fn into(self) -> (T, Counter) {
        (self.value, self.counter)
    }
}

impl<T> PartialEq for Version<T> where
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.counter == other.counter && self.value == other.value
    }
}

impl<T> fmt::Debug for Version<T> where
    T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Version {{ value={:?}, counter={:?} }}", self.value, self.counter)
    }
}

