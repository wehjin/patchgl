use self::model::Id;
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
        assert_eq!((33, Id::default()), into_tuple);
    }

    #[test]
    fn from_tuple() {
        let from_tuple = Version::from((33, Id::default()));
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
    pub id: Id,
}

impl<T> Version<T> {
    pub fn upgrades(&self, other: &Self) -> bool {
        self.id.upgrades(&other.id)
    }

    pub fn upgrades_option(&self, other: &Option<Self>) -> bool {
        match other {
            &Option::Some(ref other) => self.upgrades(other),
            &Option::None => self.id.upgrades_option(&Option::None),
        }
    }

    pub fn bump(&self, value: T) -> Self {
        Version { value, id: self.id.bump() }
    }
}

impl<T> From<(T, Id)> for Version<T> {
    fn from((value, id): (T, Id)) -> Self {
        Version { value, id }
    }
}

impl<T> From<T> for Version<T> {
    fn from(value: T) -> Self {
        Version { id: Id::default(), value }
    }
}

impl<T> Into<(T, Id)> for Version<T> {
    fn into(self) -> (T, Id) {
        (self.value, self.id)
    }
}

impl<T> PartialEq for Version<T> where
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value
    }
}

impl<T> fmt::Debug for Version<T> where
    T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Version {{ value={:?}, id={:?} }}", self.value, self.id)
    }
}

