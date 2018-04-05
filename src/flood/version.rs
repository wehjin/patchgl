use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump_increments_number_changes_value() {
        let version = Version::from(33);
        let new_version = version.bump(27);
        assert_eq!((1, 27), new_version.into());
    }

    #[test]
    fn bumped_is_greater() {
        let version = Version::from(33);
        let bumped = version.bump(27);
        assert!(bumped > version);
    }

    #[test]
    fn version_is_greater_than_none() {
        let version = Version::from(33);
        assert!(version > None);
    }
}

pub struct Version<T> {
    pub number: u64,
    pub value: T,
}

impl<T> Version<T> {
    pub fn bump(&self, value: T) -> Self {
        Version { number: self.number + 1, value }
    }
}

impl<T> PartialEq for Version<T> {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl<T> Eq for Version<T> {}

impl<T> Ord for Version<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl<T> PartialOrd for Version<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl<T> PartialEq<Option<Version<T>>> for Version<T> {
    fn eq(&self, other: &Option<Version<T>>) -> bool {
        match *other {
            Some(ref other) => self.eq(other),
            None => false,
        }
    }
}

impl<T> PartialOrd<Option<Version<T>>> for Version<T> {
    fn partial_cmp(&self, other: &Option<Version<T>>) -> Option<Ordering> {
        match *other {
            Some(ref other) => self.partial_cmp(other),
            None => Some(Ordering::Greater)
        }
    }
}

impl<T> From<(u64, T)> for Version<T> {
    fn from((number, value): (u64, T)) -> Self {
        Version { number, value }
    }
}

impl<T> From<T> for Version<T> {
    fn from(value: T) -> Self {
        Version::from((0, value))
    }
}

impl<T> Into<(u64, T)> for Version<T> {
    fn into(self) -> (u64, T) {
        (self.number, self.value)
    }
}

