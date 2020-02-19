use core::ops::{Add, Mul};
use crate::linalg::mat::Mat;
use crate::number::Sqrt;
use core::borrow::Borrow;

pub trait Norm<M: ?Sized, R> {
    fn norm(input: &M) -> R;
}

// ||x||₂
pub struct EuclideanNorm;

impl<T, U> Norm<U, T> for EuclideanNorm
where
    T: Mul<Output = T> + Add<Output = T> + Sqrt + Clone,
    U: Borrow<Mat<T>>
{
    fn norm(mat: &U) -> T {
        EuclideanNorm::norm(mat.borrow().as_slice())
    }
}

impl<T> Norm<[T], T> for EuclideanNorm
where
    T: Mul<Output = T> + Add<Output = T> + Sqrt + Clone,
{
    fn norm(slice: &[T]) -> T {
        let mut ans = slice[0].clone() * slice[0].clone();
        for i in 1..slice.len() {
            ans = ans + slice[i].clone() * slice[i].clone();
        }
        ans.sqrt()
    }
}
