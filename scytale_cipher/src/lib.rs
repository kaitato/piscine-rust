pub fn scytale_cipher(message: String, i: u32) -> String {
    let len = message.len();
    let width = (len as f32 / i as f32).ceil() as usize;
    let mut result = String::new();
    for j in 0..width {
        for k in 0..i {
            let index = (k * width as u32) + j as u32;
            if index < len as u32{
                result.push(message.chars().nth(index as usize).unwrap());
            } else {
                result.push(' ');
            }
        }
    }
    result.trim().to_string()
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
