pub fn arrange_phrase(phrase: &str) -> String {
    let mut v:Vec<&str> = phrase.split(" ").collect();
    v.sort_by_key(|word| {
        word.chars().find(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap())
    });
    let v2: Vec<String> = v.into_iter().map(|word| word.chars().filter(|c| !c.is_digit(10)).collect()).collect();
    v2.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = arrange_phrase("is2 Thi1s T4est 3a");
        println!("{:?}", result);
        assert_eq!(result, "This is a Test");
    }
}
