use rand::{Rng, SeedableRng, Isaac64Rng};

pub trait SubIds {
    fn sub_ids(&self, angle: Angle, n: usize) -> Vec<Self>
        where Self: Sized;
}

impl SubIds for u64 {
    fn sub_ids(&self, angle: Angle, n: usize) -> Vec<Self> {
        let angle: u64 = From::from(angle);
        let seed = &[*self ^ angle];
        let rng = &mut Isaac64Rng::from_seed(seed);
        let mut sub_ids = Vec::new();
        for _i in 0..n {
            let sub_id: u64 = rng.gen();
            sub_ids.push(sub_id);
        }
        sub_ids
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Angle {
    A,
    B,
}

impl From<Angle> for u64 {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::A => 11,
            Angle::B => 13,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Angle, SubIds};
    use std::collections::HashSet;

    #[test]
    fn id_emits_n_sub_ids() {
        let id = 34u64;
        let sub_ids = id.sub_ids(Angle::A, 12);
        assert_eq!(12, sub_ids.len())
    }

    #[test]
    fn id_emits_n_different_sub_ids() {
        let id = 34u64;
        let sub_ids = id.sub_ids(Angle::A, 12);

        let mut ids = HashSet::new();
        ids.insert(id);
        ids.extend(&sub_ids);
        assert_eq!(13, ids.len());
    }

    #[test]
    fn sub_ids_with_lower_n_is_subset_of_sub_ids_with_higher_n() {
        let id = 34u64;
        let sub_ids_12 = id.sub_ids(Angle::A, 12);
        let sub_ids_24 = id.sub_ids(Angle::A, 24);
        assert_eq!(sub_ids_12[..], sub_ids_24[0..12]);
    }

    #[test]
    fn sub_ids_from_different_angles_have_different_values() {
        let id = 35u64;
        let sub_ids_a = id.sub_ids(Angle::A, 1);
        let sub_ids_b = id.sub_ids(Angle::B, 1);
        assert_ne!(sub_ids_a, sub_ids_b);
    }

    #[test]
    fn sub_ids_from_different_ids_have_different_values() {
        let id1 = 34u64;
        let id2 = 35u64;
        let sub_ids_1 = id1.sub_ids(Angle::A, 1);
        let sub_ids_2 = id2.sub_ids(Angle::A, 1);
        assert_ne!(sub_ids_1, sub_ids_2);
    }
}