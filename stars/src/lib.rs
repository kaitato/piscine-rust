pub fn stars(n: u32) -> String {
    let mut str = String::new();
    let num: i32 = 2;
    for _ in 0..num.pow(n) {
        str.push('*');
    }
    str

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", stars(1));
        println!("{}", stars(4));
        println!("{}", stars(5));
    }
}
