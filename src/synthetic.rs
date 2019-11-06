use crate::array::ArrayBuf;
use crate::linalg::MatrixBuf;
use core::ops::{Mul, Add};
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;

pub fn data<T>(m: usize, n: usize) -> MatrixBuf<T> 
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

pub fn omega(m: usize, n: usize, rate: f64) -> MatrixBuf<bool> {
    let mut rng = rand::thread_rng();
    let mut a = Vec::new();
    let sample_cnt = (rate * (m as f64 * n as f64)).round() as usize;
    for _ in 0..sample_cnt {
        a.push(true)
    }
    for _ in sample_cnt..(m * n) {
        a.push(false)
    }
    a.shuffle(&mut rng);
    let mut arr = ArrayBuf::from(a);
    arr.truncate([m, n]);
    MatrixBuf::from(arr)
} 
