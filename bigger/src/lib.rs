use std::collections::HashMap;


pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let max_val = h.values().cloned().max();
    match max_val {
        Some(max) => return max,
        None => return 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let mut hash = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!("The biggest of the elements in the HashMap is {}", bigger(hash));
    }
}
