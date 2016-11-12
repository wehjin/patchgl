use cage::{Cage};

pub fn cage_from_string(cage_string: &String) -> Cage {
    let values: Vec<&str> = cage_string.split(',').collect();
    let (top_index, right_index, near_index, bottom_index, left_index) = (0, 1, 2, 3, 4);
    let z = values[near_index].trim().parse::<f32>().unwrap();
    let limits = (
        values[left_index].trim().parse::<f32>().unwrap(),
        values[right_index].trim().parse::<f32>().unwrap(),
        values[bottom_index].trim().parse::<f32>().unwrap(),
        values[top_index].trim().parse::<f32>().unwrap(),
        z, z
    );
    Cage::from(limits)
}
