pub fn num_to_ordinal(x: u32) -> String {
    let last_num = x % 10;
    let mut res = x.to_string();
    if x > 10 && x < 14 {
        res.push_str("th");
        return res
    }
    match last_num {
        1 => res.push_str("st"),
        2 => res.push_str("nd"),
        3 => res.push_str("rd"),
        _ => res.push_str("th"),
    };
    res
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(22));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
    }
}
