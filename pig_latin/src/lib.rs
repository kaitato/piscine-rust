pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiouAEIOU";
    let words: Vec<&str> = text.split(' ').collect();
    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        let mut new_word = String::new();
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            if vowels.contains(first_char) {
                new_word.push_str(&word);
                new_word.push_str("ay");
            } else {
                let mut consonants = String::new();
                consonants.push(first_char);

                while let Some(char) = chars.next() {
                    if vowels.contains(char) || (consonants.ends_with("q") && char == 'u') {
                        break;
                    }
                    consonants.push(char);
                }

                new_word.push_str(chars.as_str());
                new_word.push_str(&consonants);
                new_word.push_str("ay");
            }
        }

        if i > 0 {
            result.push(' ');
        }
        result.push_str(&new_word);
    }

    result
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
