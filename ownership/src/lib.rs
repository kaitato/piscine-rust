pub fn first_subword(s: String) -> String {
    let mut subword = String::new();
    let mut chars = s.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_uppercase() && subword.is_empty() {
            subword.push(c);
            chars.next();
        } else if c.is_uppercase() {
            break;
        } else if c == '_' {
            chars.next();
            break;
        } else {
            subword.push(c);
            chars.next();
        }
    }

    subword
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = String::from("helloWorld");
        let s2 = String::from("snake_case");
        let s3 = String::from("CamelCase");
        let s4 = String::from("just");

        println!("first_subword({}) = {}", s1.clone(), first_subword(s1));
        println!("first_subword({}) = {}", s2.clone(), first_subword(s2));
        println!("first_subword({}) = {}", s3.clone(), first_subword(s3));
        println!("first_subword({}) = {}", s4.clone(), first_subword(s4));
    }
}
