pub fn talking(text: &str) -> &str {
    // if text.chars().all(|c| c.is_uppercase()) {
    // if text.is_empty() || text.chars().all(|c| c.is_alphanumeric() == false) {
    //     return "Just say something!"
    // } else if text.chars().last() == Some('?') && text.to_uppercase() != text && text.chars().any(|c| c.is_alphanumeric()){
    //     return "Sure."
    // } else if text.to_uppercase() == text {
    //     if text.chars().last() == Some('?') {
    //         return "Quiet, I am thinking!"
    //     }else {
    //         return "There is no need to yell, calm down!"
    //     }
    // } else {
    //     return "Interesting"
    // }
    if text.is_empty() || text.chars().all(|c| c.is_alphanumeric() == false) {
        return "Just say something!"
    } else if text.chars().last() == Some('?') {
        if text.to_uppercase() == text && !text.chars().any(|c| c.is_numeric()) {
            return "Quiet, I am thinking!"
        } else {
            return "Sure."
        }
    } else if text.to_uppercase() == text {
        return "There is no need to yell, calm down!"
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
