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
    if original.is_empty() || ciphered.is_empty() {
        return Some(Err(CipherError::new(false, String::new())));
    }

    let atbash_cipher: String = original
        .chars()
        .map(|c| match c {
            'a'..='z' => (b'z' - c as u8 + b'a') as char,
            'A'..='Z' => (b'Z' - c as u8 + b'A') as char,
            _ => c,
        })
        .collect();

    if atbash_cipher == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, atbash_cipher)))
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
