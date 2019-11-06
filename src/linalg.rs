use crate::array::ArrayBuf;
use core::fmt::{self, Debug};
use core::ops::{Add, Index, IndexMut, Mul};
use core::mem;
use core::ptr;

// pub type Matrix<T, const R: usize, const C: usize>
//     = Array<T, 2, [R, C]>;

#[derive(Clone)]
pub struct MatrixBuf<T> {
    array: ArrayBuf<T>,
    transposed: bool,
}

impl<T> MatrixBuf<T> {
    pub fn nrows(&self) -> usize {
        self.array
            .shape()
            .len_of(if !self.transposed { 0 } else { 1 })
    }

    pub fn ncols(&self) -> usize {
        self.array
            .shape()
            .len_of(if !self.transposed { 1 } else { 0 })
    }
}

impl<T> MatrixBuf<T> {
    pub fn transpose(&mut self) {
        self.transposed = !self.transposed;
    }
}

impl<T> From<ArrayBuf<T>> for MatrixBuf<T> {
    fn from(src: ArrayBuf<T>) -> MatrixBuf<T> {
        if src.ndim() != 2 {
            panic!("matrices must be 2-dimension arrays");
        }
        MatrixBuf {
            array: src,
            transposed: false,
        }
    }
}

// impl<T, const R: usize, const C: usize> From<[[usize; C]; R]> for MatrixBuf<T> {
//     fn from(src: [[usize; C]; R]) -> MatrixBuf<T> {
//         let arr: [usize; R*C] = unsafe { core::mem::transmute(src) };
//         let vec = Vec::from(arr);
//         MatrixBuf {
//             array: ArrayBuf::from(vec)
//         }
//     }
// }

impl<T> Index<[usize; 2]> for MatrixBuf<T> {
    type Output = T;

    fn index(&self, [r, c]: [usize; 2]) -> &T {
        if !self.transposed {
            self.array.index([r, c])
        } else {
            self.array.index([c, r])
        }
    }
}

impl<T> IndexMut<[usize; 2]> for MatrixBuf<T> {
    fn index_mut(&mut self, [r, c]: [usize; 2]) -> &mut T {
        if !self.transposed {
            self.array.index_mut([r, c])
        } else {
            self.array.index_mut([c, r])
        }
    }
}

impl<T> fmt::Debug for MatrixBuf<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MatrixBuf")
            .field("nrows", &self.nrows())
            .field("ncols", &self.ncols())
            .field("transposed", &self.transposed)
            .field("data", &self.array.as_slice())
            .finish()
    }
}

impl<T> Mul for MatrixBuf<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone + Copy,
{
    type Output = Self;

    fn mul(self, rhs: MatrixBuf<T>) -> MatrixBuf<T> {
        gemm_scalar(self, rhs)
    }
}

// time: O(m*n*p) ~ O(n³)
fn gemm_scalar<T>(a: MatrixBuf<T>, b: MatrixBuf<T>) -> MatrixBuf<T>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    if a.ncols() != b.nrows() {
        panic!(
            "cannot multiply two matrices m*n1 and n2*p with n1 != n2 ({} != {})",
            a.ncols(),
            b.nrows()
        );
    }
    let n = b.nrows();
    let mut ans = ArrayBuf::new();
    ans.reshape_with([a.nrows(), b.ncols()], || unsafe { core::mem::zeroed() });
    for i in 0..a.nrows() {
        for j in 0..b.ncols() {
            let t1: T = unsafe { mem::MaybeUninit::uninit().assume_init() };
            let t2: T = unsafe { mem::MaybeUninit::uninit().assume_init() };
            unsafe { ptr::copy(&a[[i, 0]] as *const _, &t1 as *const _ as *mut _, 1) };
            unsafe { ptr::copy(&b[[0, j]] as *const _, &t2 as *const _ as *mut _, 1) };
            ans[[i, j]] = t1 * t2;
            for k in 1..n {
                let t1: T = unsafe { mem::MaybeUninit::uninit().assume_init() };
                let t2: T = unsafe { mem::MaybeUninit::uninit().assume_init() };
                let t3: T = unsafe { mem::MaybeUninit::uninit().assume_init() };
                unsafe { ptr::copy(&a[[i, k]] as *const _, &t1 as *const _ as *mut _, 1) };
                unsafe { ptr::copy(&b[[k, j]] as *const _, &t2 as *const _ as *mut _, 1) };
                unsafe { ptr::copy(&ans[[i, j]] as *const _, &t3 as *const _ as *mut _, 1) };
                ans[[i, j]] = t3 + t1 * t2;
            }
        }
    }
    MatrixBuf::from(ans)
}
