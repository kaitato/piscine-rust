pub fn rev_str(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut reversed = String::new();
    let mut index = input.len() - 1;
    loop {
        reversed.push(chars[index]);
        if index == 0 {
            break;
        }
        index -= 1;
    }
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
