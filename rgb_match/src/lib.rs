#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (first, second) {
            (1, 2) => {
                let temp = self.r;
                self.r = self.g;
                self.g = temp;
            }
            (1, 3) => {
                let temp = self.r;
                self.r = self.b;
                self.b = temp;
            }
            (1, 4) => {
                let temp = self.r;
                self.r = self.a;
                self.a = temp;
            }
            (2, 3) => {
                let temp = self.g;
                self.g = self.b;
                self.b = temp;
            }
            (2, 4) => {
                let temp = self.g;
                self.g = self.a;
                self.a = temp;
            }
            (3, 4) => {
                let temp = self.b;
                self.b = self.a;
                self.a = temp;
            }
            _ => {}
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };
    
        println!("{:?}", c.swap(c.r, c.b));
        println!("{:?}", c.swap(c.r, c.g));
        println!("{:?}", c.swap(c.r, c.a));
        println!();
        println!("{:?}", c.swap(c.g, c.r));
        println!("{:?}", c.swap(c.g, c.b));
        println!("{:?}", c.swap(c.g, c.a));
        println!();
        println!("{:?}", c.swap(c.b, c.r));
        println!("{:?}", c.swap(c.b, c.g));
        println!("{:?}", c.swap(c.b, c.a));
        println!();
        println!("{:?}", c.swap(c.a, c.r));
        println!("{:?}", c.swap(c.a, c.b));
        println!("{:?}", c.swap(c.a, c.g));
    }
}
