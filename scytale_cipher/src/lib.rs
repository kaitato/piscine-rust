pub fn scytale_cipher(message: String, i: u32) -> String {
    let len = message.len();
    let width = (len as f32 / i as f32).ceil() as usize;
    let mut result = vec![vec![' '; width]; i as usize];
    let mut message_chars = message.chars().peekable();
    for row in 0..width {
        for col in 0..i as usize {
            if let Some(c) = message_chars.next() {
                result[col][row] = c;
            }
        }
    }
    result.iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("scytale Code"), 6));
        println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8));
    }
}
