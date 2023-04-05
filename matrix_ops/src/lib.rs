use std::ops::{Add, Sub};

pub trait Scalar: Sized + std::ops::Add<Self, Output = Self> + std::ops::Sub<Self, Output = Self> + std::ops::Mul<Self, Output = Self> + std::ops::Div<Self, Output = Self> {
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

impl<T: Add<Output = T> + Sub<Output = T> + Clone> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Self::Output {
        let rows = self.0.len();
        let cols = self.0[0].len();
        if other.0.len() != rows || other.0[0].len() != cols {
            return None;
        }

        let mut result = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for j in 0..cols {
                row.push(self.0[i][j].clone() + other.0[i][j].clone());
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Clone> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Self::Output {
        let rows = self.0.len();
        let cols = self.0[0].len();
        if other.0.len() != rows || other.0[0].len() != cols {
            return None;
        }

        let mut result = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for j in 0..cols {
                row.push(self.0[i][j].clone() - other.0[i][j].clone());
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        println!("{:?}", matrix + matrix_2);
    
        let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
        let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
        println!("{:?}", matrix - matrix_2);
    
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        println!("{:?}", matrix - matrix_2);
    
        let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        println!("{:?}", matrix + matrix_2);
    }
}
