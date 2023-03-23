pub fn number_logic(num: u32) -> bool {
    let mut sum = 0;
    let  num_str = num.to_string();
    for n in num_str.chars() {
        sum += (n.to_digit(10).unwrap()).pow(num_str.len() as u32);
    }
    println!("num: {} sum: {}",num, sum);
    if num == sum {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let array = [9, 10, 153, 154];
    for pat in &array {
        if number_logic(*pat) == true {
            println!(
                "this number returns {} because the number {} obey the rules of the sequence",
                number_logic(*pat),
                pat
            )
        }
        if number_logic(*pat) == false {
            println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
        }
    }
    }
}
