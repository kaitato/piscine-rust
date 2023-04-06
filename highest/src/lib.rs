#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a>{
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.len() > 0{
            Some(self.numbers[self.numbers.len()-1])
        }else{
            None
        }
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut new_vec : Vec<u32> = self.numbers.clone().to_vec();
        if new_vec.len() < 3{
            new_vec.sort();
            new_vec.reverse();
            new_vec
        }else{
            new_vec.sort();
            new_vec = new_vec[new_vec.len()-3..=new_vec.len()-1].to_vec();
            new_vec.reverse();
            new_vec
        }
    }
}
