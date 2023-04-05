pub fn first_fifty_even_square() -> Vec<i32> {
    (2..=100).filter(| n | n % 2 == 0 ).map(| n | n * n).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Hello, world!");
        let v1 = first_fifty_even_square();
    
        println!("All elements in {:?}, len = {}", v1, v1.len());
    }
}
