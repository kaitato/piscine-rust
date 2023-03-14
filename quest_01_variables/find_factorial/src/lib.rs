pub fn factorial(num: u64) -> u64 {
    let mut x = num;
    let mut answer = num;
    if num == 0 {
        return 1
    }
    loop {
        x -= 1;
        if x == 0 {
            return answer
        }
        answer *= x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let factorial = factorial(1);
        assert_eq!(factorial, 1);
    }
}
