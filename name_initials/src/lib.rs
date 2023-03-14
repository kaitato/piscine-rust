pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for i in names {
        let mut initials = String::new();
        for a in i.split(" ") {
            initials.push(a.chars().next().unwrap());
            initials.push('.');
        }
        initials.insert(2, ' ');
        v.push(initials);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        println!("{:?}", initials(names));
    }
}
