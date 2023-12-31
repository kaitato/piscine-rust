pub fn capitalize_first(input: &str) -> String {
    let mut v: Vec<char> = input.chars().collect();
    if let Some(first_char) = v.first_mut() {
        *first_char = first_char.to_uppercase().next().unwrap();
    }
    v.into_iter().collect()
}


pub fn title_case(input: &str) -> String {
    let v: Vec<&str> = input.split_whitespace().collect();
    let mut res = String::new();
    for a in v {
        let line = capitalize_first(a);
        res.push_str(&line);
        res.push_str(" ");
    }
    res.trim().to_string()
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if c.is_uppercase() {
            res.push(c.to_ascii_lowercase())
        } else if c.is_lowercase() {
            res.push(c.to_ascii_uppercase())
        } else {
            res.push(c)
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", capitalize_first("joe is missing"));
        println!("{}", title_case("jill is leaving A"));
        println!("{}",change_case("heLLo THere"));
    }
}
