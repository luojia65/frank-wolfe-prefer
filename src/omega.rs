use crate::{
    array::ArrayBuf, 
    linalg::{MatrixBuf, Norm, EuclideanNorm},
    number::Sqrt,
};
use core::ops::{Add, Mul, Div};

pub trait Omega<T> {
    fn p_omega(&self, y: MatrixBuf<T>, zero: T) -> MatrixBuf<T>;

    fn pi_l_omega(&self, m: MatrixBuf<T>, l: T, one: T) -> MatrixBuf<T>;
}

impl<T> Omega<T> for MatrixBuf<bool>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Sqrt 
        + Div<Output = T> + PartialOrd,
{
    fn p_omega(&self, y: MatrixBuf<T>, zero: T) -> MatrixBuf<T> {
        let mut vec = Vec::new();
        for i in 0..y.nrows() {
            for j in 0..y.ncols() {
                vec.push(if self[[i, j]] {
                    y[[i, j]].clone()
                } else {
                    zero.clone()
                });
            }
        }
        let mut arr = ArrayBuf::from(vec);
        arr.truncate([y.nrows(), y.ncols()]);
        MatrixBuf::from(arr)
    }
    
    fn pi_l_omega(&self, m: MatrixBuf<T>, l: T, one: T) -> MatrixBuf<T> {
        let norm = EuclideanNorm::norm(&m);
        let value = if l < norm { l / norm } else { one };
        &m * value
    }
}
