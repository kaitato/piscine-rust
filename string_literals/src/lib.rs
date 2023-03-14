pub fn is_empty(v: &str) -> bool {
    for _a in v.chars() {
        return false
    }
    return true
}


pub fn is_ascii(v: &str) -> bool {
    for a in v.chars() {
        if !a.is_ascii() {
            return false
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
     if v.find(pat) == None {
        return false
     };
     true
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let str1 = &v[..index];
    let str2 = &v[index..];
    let tup: (&str, &str) = (&str1, &str2);
    tup
}

pub fn find(v: &str, pat: char) -> usize {
    v.chars().position(|v| v == pat).unwrap()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = is_empty("");
//         assert_eq!(result, true);
//         let result1 = is_ascii("rust");
//         assert_eq!(result1, true);
//         let result2 = contains("rust", "ru");
//         assert_eq!(result2, true);
//         let result3 = split_at("rust", 2);
//         assert_eq!(result3, "ru", "st");
//         let result4 = find("rust", "u");
//         assert_eq!(result4, 1);
//     }
// }
