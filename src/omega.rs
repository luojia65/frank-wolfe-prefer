use crate::{
    linalg::{MatBuf, Norm, EuclideanNorm},
    number::Sqrt,
};
use core::ops::{Add, Mul, Div};

pub trait Omega<T> {
    fn p_omega(&self, y: MatBuf<T>, zero: T) -> MatBuf<T>;

    fn pi_l_omega(&self, m: MatBuf<T>, l: T, one: T) -> MatBuf<T>;
}

impl<T> Omega<T> for MatBuf<bool>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Sqrt 
        + Div<Output = T> + PartialOrd,
{
    fn p_omega(&self, y: MatBuf<T>, zero: T) -> MatBuf<T> {
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
        let mut ans = MatBuf::from(vec);
        ans.reshape([y.nrows(), y.ncols()]);
        ans
    }
    
    fn pi_l_omega(&self, m: MatBuf<T>, l: T, one: T) -> MatBuf<T> {
        let norm = EuclideanNorm::norm(&m);
        let value = if l < norm { l / norm } else { one };
        m * value
    }
}
