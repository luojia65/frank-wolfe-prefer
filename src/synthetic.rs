use crate::linalg::MatBuf;
use core::ops::{Add, Mul};
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;

pub fn data<T>(m: usize, n: usize) -> MatBuf<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
    Standard: Distribution<T>,
{
    let mut uv = Vec::new();
    for _ in 0..m {
        uv.push(rand::random());
    }
    let mut u = MatBuf::from(uv);
    u.reshape([m, 1]);
    let mut vv = Vec::new();
    for _ in 0..n {
        vv.push(rand::random());
    }
    let mut v = MatBuf::from(vv);
    v.reshape([1, n]);
    u * v
}

pub fn omega(m: usize, n: usize, rate: f64) -> MatBuf<bool> {
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
    let mut ans = MatBuf::from(a);
    ans.reshape([m, n]);
    ans
}
