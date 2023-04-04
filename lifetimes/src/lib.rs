#[derive(Debug, PartialEq, Eq)]
pub struct Person<'a>{
	pub name: &'a str,
	pub age: u8,
}

impl Person<'_> {
	pub fn new(name: &str) -> Person {
        Person { name, age: 0 }
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let person = Person::new("Leo");

        println!("Person = {:?}", person);
    }
}
