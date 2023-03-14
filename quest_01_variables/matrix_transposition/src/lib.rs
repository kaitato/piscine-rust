#[derive(Debug)]

pub struct Matrix(pub (i32,i32),pub (i32,i32));

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

pub fn transpose(m: Matrix) -> Matrix {
    return Matrix((m.0.0, m.1.0), (m.0.1, m.1.1))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let m = transpose((0, 0), (0, 0));
//         assert_eq!(m, Matrix((0, 0), (0, 0)));
//     }
// }
