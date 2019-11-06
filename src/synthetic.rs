use crate::array::ArrayBuf;
use crate::linalg::MatrixBuf;
use core::ops::{Mul, Add};
use rand::distributions::{Distribution, Standard};

pub fn generate<T>(m: usize, n: usize) -> MatrixBuf<T> 
where 
    T: Mul<Output = T> + Add<Output = T>,
    Standard: Distribution<T> 
{
    let mut uv = Vec::new();
    for _ in 0..m {
        uv.push(rand::random());
    }
    let mut ua = ArrayBuf::from(uv);
    ua.truncate([m, 1]);
    let u = MatrixBuf::from(ua);
    let mut vv = Vec::new();
    for _ in 0..n {
        vv.push(rand::random());
    }
    let mut va = ArrayBuf::from(vv);
    va.truncate([1, n]);
    let v = MatrixBuf::from(va);
    u * v
}
