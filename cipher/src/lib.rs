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
    let atbash_cipher = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let diff = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                (b'z' - (c as u8 - diff)) as char
            } else {
                c
            }
        })
        .collect::<String>();
    if atbash_cipher == ciphered {
        Some(Ok(true))
    } else {
        let expected_cipher = atbash_cipher.clone();
        let err = CipherError::new(false, expected_cipher);
        Some(Err(err))
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
