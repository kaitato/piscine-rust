pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = Vec::new();

    for c in s.chars() {
         if c == '-' {
            chars.pop();
        } else {
            chars.push(c);
        }
    }
    s.clear();
    for c in chars {
        s.push(c);
    } 
    let mut chars: Vec<char> = Vec::new();
    for c in s.chars().rev() {
        if c == '+' {
            chars.pop();
        } else {
            chars.push(c)
        }
    }
    s.clear();
    for c in chars {
        s.push(c);
    } 
    let reversed: String = s.chars().rev().collect();
    *s = reversed
}


pub fn do_operations(v: &mut Vec<String>) {
    let mut sum = 0;
    for eq in v.iter_mut() {
        if eq.contains('+') {
            for num_str in eq.split('+') {
                sum += num_str.trim().parse::<i32>().unwrap();
            }
            *eq = sum.to_string();
        } else if eq.contains('-') {
            let nums: Vec<i32> = eq.split('-').map(|num_str| num_str.trim().parse::<i32>().unwrap()).collect();
            sum = nums[0] - nums[1];
            *eq = sum.to_string();
        } else {
            sum = eq.parse().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = String::from("bpp--o+er+++sskroi-++lcw");
        let mut b: Vec<String> = vec!["2+2", "3+2", "10-3", "5+5"];
    
        delete_and_backspace(&mut a);
        do_operations(&mut b);
    
        println!("{:?}", (a, b));
    }
}
