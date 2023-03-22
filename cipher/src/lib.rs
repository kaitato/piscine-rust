#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    validation: bool,
    expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError { validation, expected }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let atbash = original.chars()
                         .map(|c| {
                             if c.is_ascii_alphabetic() {
                                 (b'a' + b'z' - c.to_ascii_lowercase() as u8) as char
                             } else {
                                 c
                             }
                         })
                         .collect::<String>();

    if atbash == ciphered {
        Some(Ok(true))
    } else {
        let error = CipherError::new(false, atbash);
        Some(Err(error))
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
