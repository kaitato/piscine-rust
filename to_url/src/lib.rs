pub fn to_url(s: &str) -> String  {
    // let bytes = s.as_bytes();
    let mut string = String:: from("");
    for a in s.chars() {
        if a == ' ' {
            string.push_str("%20");
        } else {
            string.push(a);

        }
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        let s = "Hello, world!";
        println!("{} to be use as an url is {}", s, to_url(s));    }
}
