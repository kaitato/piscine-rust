pub fn spell(n: u64) -> String {
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let teens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let thousands = ["", "thousand", "million"];
    let mut n = n;
    if n < 10 {
        return ones[n as usize].to_string();
    } else if n < 20 {
        return teens[(n - 10) as usize].to_string();
    } else if n < 100 {
        let digit1 = n / 10;
        let digit2 = n % 10;
        let mut result = tens[digit1 as usize].to_string();
        if digit2 != 0 {
            result.push_str("-");
            result.push_str(ones[digit2 as usize]);
        }
        return result;
    } else {
        let mut result = String::new();
        let mut i = 0;
        while n > 0 {
            let group = n % 1000;
            if group > 0 {
                let mut group_str = spell(group);
                if i > 0 {
                    group_str.push_str(" ");
                    group_str.push_str(thousands[i]);
                }
                if !result.is_empty() {
                    result.insert_str(0, " ");
                }
                result.insert_str(0, &group_str);
            }
            n /= 1000;
            i += 1;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{}", spell(348));
    println!("{}", spell(9996));
    }
}
