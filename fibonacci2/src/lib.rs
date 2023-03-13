pub fn fibonacci(n: u32) -> u32 {

    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else if n > 1 {
        return fibonacci(n-1) + fibonacci(n-2)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
