use std::{ops::Mul};
// use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);


impl<T: Copy + Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> &[T] {
        &self.0[n]
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut new_vec: Vec<&T> = Vec::new();
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if j == n {
                    new_vec.push(&self.0[i][n])
                }
            }
        }
        let mut col = Vec::new();
        for &v in &new_vec.clone() {
            col.push(*v)
        }
        col
    }
}
impl<T: Mul<Output = T> + Clone + std::iter::Sum> Mul for Matrix<T> {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        };

        for (i, arr) in self.0.iter().enumerate() {
            if arr.len() != rhs.0[i].len() {
                return None;
            }
        }
        let mut m_vec : Vec<Vec<T>> = Vec::new();
        let mut mult_row = 0;
        while mult_row < self.0.len() {
            let mut mult_col = 0;
            let mut new_vec: Vec<T> = Vec::new();
            let mut row_vec: Vec<T> = Vec::new();
            for t in &self.0[mult_row] {
                row_vec.push(t.clone())
            }
            while mult_col < rhs.0.len() {
                let mut col_vec: Vec<T> = Vec::new();
                for t in &rhs.0 {
                    col_vec.push(t[mult_col].clone());
                }
                new_vec.push(col_vec.into_iter().zip(row_vec.clone()).map(|(x,y)| x * y).sum());
                mult_col += 1;
            }
            m_vec.push(new_vec);
            mult_row += 1;
        };
        Some(Matrix(m_vec))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        println!("{:?}", matrix.col(0));
        println!("{:?}", matrix.row(1));
    
        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
        let mult = matrix_1.clone() * matrix_2.clone();
        println!("{:?}", mult);
        println!("{:?}", matrix_1.number_of_cols());
        println!("{:?}", matrix_2.number_of_rows());
    }
}
