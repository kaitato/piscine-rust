use core::num;

use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
    Num(u32),
}

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            1..=4 => I,
            5..=9 => V,
            10..=49 => X,
            50..=99 => L,
            100..=499 => C,
            500..=999 => D,
            1000..=5000 => M,
            _ => Nulla,
        }
    }
}

#[derive(Debug)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut quotient = n;
        let mut exponent = 0;
        let mut reverse_roman = Vec::new();

        while quotient != 0 {
            let rest = quotient % 10;
            quotient /= 10;
            exponent += 1;
            if rest == 9 {
                reverse_roman.push(RomanDigit::from(10_u32.pow(exponent)));
                reverse_roman.push(RomanDigit::from(10_u32.pow(exponent - 1)));
            } else if rest == 4 {
                reverse_roman.push(RomanDigit::from(10_u32.pow(exponent) / 2));
                reverse_roman.push(RomanDigit::from(10_u32.pow(exponent - 1)));
            } else if rest >= 5 {
                let repetitions = rest - 5;
                for _ in 0..repetitions {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(exponent - 1)));
                }
                reverse_roman.push(RomanDigit::from(10_u32.pow(exponent) / 2));
            } else {
                for _ in 0..rest {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(exponent - 1)))
                }
            }
        }
        reverse_roman.reverse();
        reverse_roman.push(RomanDigit::Num(n));
        RomanNumber(reverse_roman)
    }
}

impl Iterator for RomanNumber {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() > 0 {
            let n = self.0.clone()[self.0.len() - 1];
            let mut number: u32 = 0;
            match n {
                RomanDigit::Num(num) => number = num,
                _ => number = 0,
            };
            let mut new_r = RomanNumber::from(number + 1);
            self.0.pop();
            new_r.0.pop();
            new_r.0.push(RomanDigit::Num(number + 1));
            Some(new_r)
        } else {
            None
        }
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
