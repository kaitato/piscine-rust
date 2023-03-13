pub fn rev_str(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rev = rev_str("Hello world");
        assert_eq!(result, "dlrow olleH");
    }
}
