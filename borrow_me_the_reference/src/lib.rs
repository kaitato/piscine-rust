pub fn delete_and_backspace(s: &mut String) {
    let mut result_string = String::new();
    let mut chars: Vec<char> = Vec::new();
    let mut skip_next_char = false;

    
    for c in s.chars() {
        if skip_next_char {
            skip_next_char = false;
            continue;
        }
        match c {
            '-' => {
                chars.pop();
            },
            '+' => {
                skip_next_char = true;
            },
            _ => {
                chars.push(c);
            }
        }
    }
    for c in chars {
        result_string.push(c);
    }
    println!("{}",result_string);
}

// pub fn do_operations(v: &mut Vec<String>) {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = String::from("bpp--o+er+++sskroi-++lcw");
        // let mut b: Vec<String> = vec!["2+2", "3+2", "10-3", "5+5"];
    
        delete_and_backspace(&mut a);
        // do_operations(&mut b);
    
        // println!("{:?}", (a, b));
    }
}
