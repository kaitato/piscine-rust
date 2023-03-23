pub fn rotate(input: &str, key: i8) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut res = String::new();
    if key == 0 {
        return input.to_string()
    }
    for char in input.chars() {
        match char {
            'a'..='z' | 'A'..='Z' => {
                let index = alphabet.find(char.to_ascii_lowercase()).unwrap();
                let rotated_index = (index as i8 + key + 26) % 26;
                let rotated_char = alphabet.chars().nth(rotated_index as usize).unwrap();
                res.push(if char.is_ascii_uppercase() { rotated_char.to_ascii_uppercase() } else { rotated_char });
            }
            _ => {
                // non-letter characters are not rotated
                res.push(char);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    println!("The letter \"a\" becomes: {}", rotate("a", 26));
    println!("The letter \"m\" becomes: {}", rotate("m", 0));
    println!("The letter \"m\" becomes: {}", rotate("m", 13));
    println!("The letter \"a\" becomes: {}", rotate("a", 15));
    println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
    println!(
        "The decoded message is: {}",
        rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
    );
    println!(
        "The decoded message is: {}",
        rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
    );
    println!(
        "Your cypher wil be: {}",
        rotate("Testing with numbers 1 2 3", 4)
    );
    println!("Your cypher wil be: {}", rotate("Testing", -14));
    println!("The letter \"a\" becomes: {}", rotate("a", -1));
    }
}
