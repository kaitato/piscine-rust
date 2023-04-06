use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::cmp::Eq + std::hash::Hash, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let min_len = std::cmp::min(keys.len(), values.len());
    let key_slice = &keys[..min_len];
    let value_slice = &values[..min_len];
    key_slice.iter().zip(value_slice.iter()).collect::<HashMap<_, _>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2];
        println!("{:?}", slices_to_map(&keys, &values));
    }
}
