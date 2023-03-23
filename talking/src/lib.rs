pub fn talking(text: &str) -> &str {
    // if text.chars().all(|c| c.is_uppercase()) {
    if text.is_empty() || text.chars().all(|c| c.is_alphanumeric() == false) {
        return "Just say something!"
    } else if text.chars().filter(|c| c.is_alphabetic()).all(|c|c.is_ascii_uppercase()) && text.chars().any(|c| !c.is_alphanumeric()){
        if text.chars().last() == Some('?') {
            return "Quiet, I am thinking!"
        }else {
            return "There is no need to yell, calm down!"
        }
    } else if text.chars().last() == Some('?') {
        return "Sure."
    } else {
        return "Interesting"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", talking("JUST DO IT!"));
        println!("{:?}", talking("Hello how are you?"));
        println!("{:?}", talking("WHAT'S GOING ON?"));
        println!("{:?}", talking("something"));
        println!("{:?}", talking(""));
    }
}
