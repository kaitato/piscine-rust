use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut counts1 = HashMap::new();
    for c in s1.chars() {
        *counts1.entry(c).or_insert(0) += 1;
    }
    for c in s2.chars() {
        let count = counts1.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let word = "thought";
	let word1 = "thougth";
	println!(
		"Is `{}` a permutation of `{}`? = {}",
		word,
		word1,
		is_permutation(word, word1)
	);
    }
}
