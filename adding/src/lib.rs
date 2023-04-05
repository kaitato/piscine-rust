pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
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
