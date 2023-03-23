pub fn is_pangram(s: &str) -> bool {
    let str = s.to_ascii_lowercase();
    for c in 'a'..='z' {
        if !str.contains(c) {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            is_pangram("the quick brown fox jumps over the lazy dog!")
        );
        println!("{}", is_pangram("this is not a pangram!"));
    }
}
