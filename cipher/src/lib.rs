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
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (b'z' - (c as u8 - base)) as char
            } else {
                c
            }
        })
        .collect::<String>();

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
