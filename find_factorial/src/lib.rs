pub fn factorial(num: u64) -> u64 {
    let mut x = 1;
    let mut y = num;
    loop {
        x *= y;
        y -= 1;
        if y == 1 {
            return x
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let factorial = factorial(6);
        assert_eq!(factorial, 720);
    }
}
