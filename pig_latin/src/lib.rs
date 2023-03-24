pub fn pig_latin(word: &str) -> String {
    let vowels = "aeiouAEIOU";
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    let mut rest = chars.as_str();
    let mut pig_word = String::new();
    if vowels.contains(first_char) {
        pig_word.push_str(word);
        pig_word.push_str("ay");
    } else if rest.starts_with("qu") {
        let rest = &chars.as_str().replace("qu", "");
        pig_word.push_str(rest);
        pig_word.push(first_char);
        pig_word.push_str("quay");
    } else {
        let mut consonants = String::new();
        consonants.push(first_char);
        while let Some(c) = chars.next() {
            if c == 'q' {
                // print!("hi?");
                if let Some(next_char) = chars.next() {
                    if next_char == 'u' {
                        consonants.push_str("qu");
                        pig_word.push_str(rest);
                        pig_word.push_str(&consonants);
                        pig_word.push_str("ay");
                        break;
                    }
                }
            }
            if !vowels.contains(c) {
                consonants.push(c);
            } else {
                // print!("---{}---",rest);
                // rest = chars.as_str();
                pig_word.push_str(rest);
                pig_word.push_str(&consonants);
                pig_word.push_str("ay");
                break;
            }
        }
    }
    pig_word
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", pig_latin(&String::from("igloo")));
        println!("{}", pig_latin(&String::from("apple")));
        println!("{}", pig_latin(&String::from("hello")));
        println!("{}", pig_latin(&String::from("square")));
        println!("{}", pig_latin(&String::from("xenon")));
        println!("{}", pig_latin(&String::from("chair")));
        println!("{}", pig_latin(&String::from("queen")));
    }
}
