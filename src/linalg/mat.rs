use core::{
    borrow::Borrow,
    slice::{from_raw_parts, from_raw_parts_mut},
    mem::transmute,
    fmt::{self, Debug},
    ops::{Deref, Index},
};
use std::borrow::ToOwned;

#[derive(Eq, PartialEq, Hash)]
pub struct Mat<T> {
    // len(slice) = nrows + (ncols << (sizeof(usize)/2))
    slice: [T] 
}

const HALF_USIZE_BITS: usize = core::mem::size_of::<usize>() * 8 / 2;
const HALF_LOW_MASK: usize = (1 << HALF_USIZE_BITS) - 1;
const HALF_HIGH_MASK: usize = !HALF_LOW_MASK;

impl<T> Mat<T> {
    pub fn new(src: &[T], nrows: usize, ncols: usize) -> &Mat<T> {
        let shape = nrows + (ncols << HALF_USIZE_BITS);
        let slice = unsafe { from_raw_parts(src.as_ptr(), shape) };
        unsafe { transmute(slice) }
    }

    pub fn nrows(&self) -> usize {
        self.slice.len() & HALF_LOW_MASK
    }

    pub fn ncols(&self) -> usize {
        (self.slice.len() & HALF_HIGH_MASK) >> HALF_USIZE_BITS
    }

    pub fn len(&self) -> usize {
        self.nrows() * self.ncols()
    }
    
    pub fn shape(&self) -> [usize; 2] {
        [self.nrows(), self.ncols()]
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { from_raw_parts(self.slice.as_ptr(), self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { from_raw_parts_mut(self.slice.as_mut_ptr(), self.len()) }
    }

    pub fn as_ptr(&self) -> *const T {
        self.slice.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.slice.as_mut_ptr()
    }
}

impl<T: Debug> Debug for Mat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Mat")
            .field("nrows", &self.nrows())
            .field("ncols", &self.ncols())
            .field("slice", &self.as_slice())
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MatBuf<T> {
    shape: [usize; 2],
    buf: Vec<T>,
}

impl<T> MatBuf<T> {
    pub fn new() -> MatBuf<T> {
        MatBuf {
            shape: [0, 0],
            buf: Vec::new()
        }
    }

    pub fn with_capacity(cap: usize) -> MatBuf<T> {
        MatBuf {
            shape: [0, 0],
            buf: Vec::with_capacity(cap)
        }
    }

    pub fn as_slice(&self) -> &[T] {
        self.buf.as_slice()
    }
    
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.buf.as_mut_slice()
    }

    pub fn as_ptr(&self) -> *const T {
        self.buf.as_ptr()
    }
    
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.buf.as_mut_ptr()
    }

    pub fn nrows(&self) -> usize {
        self.shape[0]
    }

    pub fn ncols(&self) -> usize {
        self.shape[1]
    }

    pub fn len(&self) -> usize {
        self.shape[0] * self.shape[1]
    }

    pub fn reshape(&mut self, shape: [usize; 2]) {
        assert_eq!(self.buf.len(), shape[0] * shape[1]);
        self.shape = shape;
    }

    pub fn shape(&self) -> [usize; 2] {
        self.shape
    }
}

impl<T: Clone> MatBuf<T> {
    pub fn new_filled(t: T, shape: [usize; 2]) -> Self {
        let vec = vec![t; shape[0] * shape[1]];
        MatBuf {
            buf: vec,
            shape
        }
    } 
}

impl<T: Clone> MatBuf<T> {
    pub fn transpose(&mut self) {
        let mut new_vec = Vec::new();
        for j in 0..self.ncols() {
            for i in 0..self.nrows() {
                new_vec.push(self[[i, j]].clone());
            }
        }
        self.buf = new_vec;
        self.reshape([self.ncols(), self.nrows()]);
    }
}

impl<T> From<Vec<T>> for MatBuf<T> {
    fn from(src: Vec<T>) -> MatBuf<T> {
        MatBuf { 
            shape: [0, 0],
            buf: src
        }
    }
}

impl<T> AsRef<Mat<T>> for MatBuf<T> {
    fn as_ref(&self) -> &Mat<T> {
        Mat::new(&self.buf, self.nrows(), self.ncols())
    }
}

impl<T> Borrow<Mat<T>> for MatBuf<T> {
    fn borrow(&self) -> &Mat<T> {
        self.as_ref()
    }
}

impl<T: Clone> ToOwned for Mat<T> {
    type Owned = MatBuf<T>;
    fn to_owned(&self) -> MatBuf<T> {
        MatBuf { 
            shape: [self.nrows(), self.ncols()],
            buf: self.as_slice().to_vec()
        }
    }
}

impl<T> Deref for MatBuf<T> {
    type Target = Mat<T>;

    fn deref(&self) -> &Mat<T> {
        self.as_ref()
    }
}

impl<T: PartialEq> PartialEq<MatBuf<T>> for Mat<T> {
    fn eq(&self, other: &MatBuf<T>) -> bool {
        self.nrows() == other.nrows() && self.ncols() == other.ncols() &&
        self.as_slice() == other.as_slice()
    }
}

impl<T: PartialEq> PartialEq<MatBuf<T>> for &Mat<T> {
    fn eq(&self, other: &MatBuf<T>) -> bool {
        *self == other
    }
}

impl<T: PartialEq> PartialEq<Mat<T>> for MatBuf<T> {
    fn eq(&self, other: &Mat<T>) -> bool {
        other == self
    }
}
impl<T: PartialEq> PartialEq<&Mat<T>> for MatBuf<T> {
    fn eq(&self, other: &&Mat<T>) -> bool {
        *other == self
    }
}

impl<T> Index<[usize; 2]> for Mat<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &T {
        let [i, j] = index;
        &self.as_slice()[self.ncols() * i + j]
    }
}

impl<T> Index<usize> for Mat<T> {
    type Output = [T]; // MatrixView<T>

    fn index(&self, r: usize) -> &[T] {
        let cols = self.ncols();
        &self.as_slice()[(cols * r)..(cols * (r + 1))]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_eq() {
        let src = &[1, 2, 3, 4, 5, 6]; 
        let mat = Mat::new(src, 2, 3);
        let mat_buf = mat.to_owned();
        assert_eq!(mat, mat);
        assert_eq!(mat_buf, mat_buf);
        assert_eq!(mat, mat_buf);
        assert_eq!(*mat, mat_buf);
        assert_eq!(mat_buf, mat);
        assert_eq!(mat_buf, *mat);
    }

    #[test]
    fn matrix_as_ref() {
        let src = &[1, 2, 3, 4, 5, 6]; 
        let mat = Mat::new(src, 2, 3);
        let mat_buf = mat.to_owned();
        fn accepts_mat_ref<T>(mat: &Mat<T>) -> usize {
            mat.len()
        }
        // must compiles
        assert_eq!(accepts_mat_ref(&mat), 6);
        assert_eq!(accepts_mat_ref(&mat_buf), 6); 
    }
}
