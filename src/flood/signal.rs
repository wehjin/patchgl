use ::flood::Version;

#[derive(Debug)]
pub struct Signal<MsgT> {
    pub id: u64,
    pub version: Version<MsgT>,
}

impl<MsgT> Signal<MsgT> where MsgT: Clone {
    pub fn clone_value(&self) -> MsgT {
        self.version.value.clone()
    }

    pub fn upgrades_option(&self, other: &Option<&Self>) -> bool {
        match other {
            &Option::Some(ref other) => self.version.upgrades(&other.version),
            &Option::None => self.version.upgrades_option(&Option::None),
        }
    }
}

impl<MsgT> Clone for Signal<MsgT> where MsgT: Clone {
    fn clone(&self) -> Self {
        Signal { id: self.id, version: self.version.clone() }
    }
}

impl<MsgT> From<(u64, Version<MsgT>)> for Signal<MsgT> where MsgT: Clone {
    fn from((id, version): (u64, Version<MsgT>)) -> Self {
        Signal { id, version: Version::from(version) }
    }
}
