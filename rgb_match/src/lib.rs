#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}



impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }

    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let first_color = match first {
            1 => self.r(),
            2 => self.g(),
            3 => self.b(),
            4 => self.a(),
            _ => unreachable!(),
        };
        let second_color = match second {
            1 => self.r(),
            2 => self.g(),
            3 => self.b(),
            4 => self.a(),
            _ => unreachable!(),
        };
        match (first, second) {
            (1, 2) => {
                self.set_r(second_color);
                self.set_g(first_color);
            }
            (1, 3) => {
                self.set_r(second_color);
                self.set_b(first_color);
            }
            (1, 4) => {
                self.set_r(second_color);
                self.set_a(first_color);
            }
            (2, 3) => {
                self.set_g(second_color);
                self.set_b(first_color);
            }
            (2, 4) => {
                self.set_g(second_color);
                self.set_a(first_color);
            }
            (3, 4) => {
                self.set_b(second_color);
                self.set_a(first_color);
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
