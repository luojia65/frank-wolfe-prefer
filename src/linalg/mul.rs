use core::ops::{Add, Mul, MulAssign};
use super::{Mat, MatBuf};
use core::borrow::Borrow;

impl<T> Mul<T> for MatBuf<T>
where T: Mul<Output = T> + Clone {
    type Output = MatBuf<T>;
    
    fn mul(mut self, rhs: T) -> Self::Output {
        for n in self.as_mut_slice().iter_mut() {
            *n = n.clone() * rhs.clone()
        }
        self
    }
}

impl<T> Mul<MatBuf<T>> for MatBuf<T> 
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
{
    type Output = MatBuf<T>;

    fn mul(self, rhs: MatBuf<T>) -> Self::Output {
        gemm_scalar(&self, &rhs)
    }
}

impl<'a, T> Mul<&'a Mat<T>> for MatBuf<T> 
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
{
    type Output = MatBuf<T>;

    fn mul(self, rhs: &'a Mat<T>) -> Self::Output {
        gemm_scalar(&self, rhs)
    }
}

impl<'a, T, U> Mul<U> for &'a Mat<T> 
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
    U: Borrow<Mat<T>>,
{
    type Output = MatBuf<T>;

    fn mul(self, rhs: U) -> Self::Output {
        gemm_scalar(&self, rhs.borrow())
    }
}

impl<T, U> MulAssign<U> for MatBuf<T> 
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
    U: Borrow<Mat<T>>
{
    fn mul_assign(&mut self, other: U) {
        *self = gemm_scalar(self, other.borrow())
    }
}

// time: O(m*n*p) ~ O(n³)
pub fn gemm_scalar<T>(a: &Mat<T>, b: &Mat<T>) -> MatBuf<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
{
    check_for_mul(a, b);
    let n = b.nrows();
    let mut vec = Vec::new();
    for i in 0..a.nrows() {
        for j in 0..b.ncols() {
            let mut ans = a[[i, 0]].clone() * b[[0, j]].clone();
            for k in 1..n {
                ans = ans + a[[i, k]].clone() * b[[k, j]].clone();
            }
            vec.push(ans)
        }
    }
    let mut ret = MatBuf::from(vec);
    ret.reshape([a.nrows(), b.ncols()]);
    ret
}

#[inline]
fn check_for_mul<T>(a: &Mat<T>, b: &Mat<T>) {
    if a.ncols() != b.nrows() {
        panic!(
            "cannot multiply two matrices m*n1 and n2*p while n1 != n2 ({} != {})",
            a.ncols(),
            b.nrows()
        );
    }
}
