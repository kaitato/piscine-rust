#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError { validation, expected }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let ciphered_lower = ciphered.to_lowercase();
    let original_lower = original.to_lowercase().chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| ('a' as u8 + ('z' as u8 - c as u8)) as char)
        .collect::<String>();
    if ciphered_lower == original_lower {
        Some(Ok(true))
    }else {
        // let expected = original_lower;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
    println!("{:?}", cipher("", "svool"));
    }
}
