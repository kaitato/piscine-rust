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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Invalid Roman Character")
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(RomanNumber::from(3).0, [I, I, I]);
        assert_eq!(RomanNumber::from(6).0, [V, I]);
        assert_eq!(RomanNumber::from(15).0, [X, V]);
        assert_eq!(RomanNumber::from(30).0, [X, X, X]);
        assert_eq!(RomanNumber::from(150).0, [C, L]);
        assert_eq!(RomanNumber::from(200).0, [C, C]);
        assert_eq!(RomanNumber::from(600).0, [D, C]);
        assert_eq!(RomanNumber::from(1500).0, [M, D]);
    }

    #[test]
    fn substractive_notation() {
        assert_eq!(RomanNumber::from(4).0, [I, V]);
        assert_eq!(RomanNumber::from(44).0, [X, L, I, V]);
        assert_eq!(RomanNumber::from(3446).0, [M, M, M, C, D, X, L, V, I]);
        assert_eq!(RomanNumber::from(9).0, [I, X]);
        assert_eq!(RomanNumber::from(94).0, [X, C, I, V]);
    }
}
