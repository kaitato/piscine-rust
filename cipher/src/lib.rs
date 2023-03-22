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
    let atbash = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let a = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let z = if c.is_ascii_uppercase() { b'Z' } else { b'z' };
                (z - (c as u8) + a) as char
            } else {
                c
            }
        })
        .collect::<String>();

    if atbash == ciphered {
        Some(Ok(true))
    } else {
        let expected = atbash.clone();
        Some(Err(CipherError::new(false, expected)))
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
