use core::ops::{Add, AddAssign, Sub, SubAssign};
use super::{Mat, MatBuf};
use core::borrow::Borrow;

impl<'a, T: AddAssign + Clone, U: Borrow<Mat<T>>> AddAssign<U> for &'a mut Mat<T> {
    fn add_assign(&mut self, other: U) {
        let b = other.borrow();
        assert_eq!(self.shape(), b.shape());
        let slice = b.as_slice();
        for (i, n) in self.as_mut_slice().iter_mut().enumerate() {
            *n += slice[i].clone()
        }
    }
}

impl<T: AddAssign + Clone, U: Borrow<Mat<T>>> AddAssign<U> for MatBuf<T> {
    fn add_assign(&mut self, other: U) {
        let b = other.borrow();
        assert_eq!(self.shape(), b.shape());
        let slice = b.as_slice();
        for (i, n) in self.as_mut_slice().iter_mut().enumerate() {
            *n += slice[i].clone()
        }
    }
}

impl<T: Add<Output = T> + Clone, U: Borrow<Mat<T>>> Add<U> for MatBuf<T> {
    type Output = MatBuf<T>;
    fn add(mut self, rhs: U) -> Self::Output {
        let b = rhs.borrow();
        assert_eq!(self.shape(), b.shape());
        let slice = b.as_slice();
        for (i, n) in self.as_mut_slice().iter_mut().enumerate() {
            *n = n.clone() + slice[i].clone()
        }
        self
    }
}

impl<'a, T: SubAssign + Clone, U: Borrow<Mat<T>>> SubAssign<U> for &'a mut Mat<T> {
    fn sub_assign(&mut self, other: U) {
        let b = other.borrow();
        assert_eq!(self.shape(), b.shape());
        let slice = b.as_slice();
        for (i, n) in self.as_mut_slice().iter_mut().enumerate() {
            *n -= slice[i].clone()
        }
    }
}

impl<T: SubAssign + Clone, U: Borrow<Mat<T>>> SubAssign<U> for MatBuf<T> {
    fn sub_assign(&mut self, other: U) {
        let b = other.borrow();
        assert_eq!(self.shape(), b.shape());
        let slice = b.as_slice();
        for (i, n) in self.as_mut_slice().iter_mut().enumerate() {
            *n -= slice[i].clone()
        }
    }
}

impl<T: Sub<Output = T> + Clone, U: Borrow<Mat<T>>> Sub<U> for MatBuf<T> {
    type Output = MatBuf<T>;
    fn sub(mut self, rhs: U) -> Self::Output {
        let b = rhs.borrow();
        assert_eq!(self.shape(), b.shape());
        let slice = b.as_slice();
        for (i, n) in self.as_mut_slice().iter_mut().enumerate() {
            *n = n.clone() - slice[i].clone()
        }
        self
    }
}
