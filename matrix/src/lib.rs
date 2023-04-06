// use std::ops::{Index, IndexMut};
pub trait Scalar: Sized{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self::Item {
        0.
    }
    fn one() -> Self::Item {
        1.
    }
}
impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item {
        0.
    }
    fn one() -> Self::Item {
        1.
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
	}
	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut new_vec: Vec<Vec<T>> = Vec::new();
        for _ in 0..row {
            let mut new_row: Vec<T> = Vec::new();
            for _ in 0..col {
                new_row.push(T::zero());
            }
            new_vec.push(new_row);
        }
        Matrix(new_vec)
	}
	pub fn identity(n: usize) -> Matrix<T> {
        let mut new_vec: Vec<Vec<T>> = Vec::new();
        for i in 0..n {
            let mut new_row: Vec<T> = Vec::new();
            for j in 0..n {
                if i == j {
                    new_row.push(T::one())
                } else {
                    new_row.push(T::zero())
                }
            }
            new_vec.push(new_row)
        }
        Matrix(new_vec)
	}
}
// pub fn identity(n: usize) -> Matrix<T> {
//     let mut matrix = Matrix::zero(n, n);
//     for i in 0..n {
//         matrix[i][i] = T::one();
//     }
//     matrix
// }
// impl<T> Index<usize> for Matrix<T> {
//     type Output = Vec<T>;
//     fn index(&self, i: usize) -> &Vec<T> {
//         &self.0[i]
//     }
// }
// impl<T> IndexMut<usize> for Matrix<T> {
//     fn index_mut(&mut self, i: usize) -> &mut Vec<T> {
//         &mut self.0[i]
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
	println!("{:?}", m);
	println!("{:?}", Matrix::<i32>::identity(4));
	println!("{:?}", Matrix::<f64>::zero(3, 4));
    }
}
