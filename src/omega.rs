use crate::{array::ArrayBuf, linalg::MatrixBuf};

pub trait Omega<T> {
    fn p_omega(&self, y: MatrixBuf<T>, zero: T) -> MatrixBuf<T>;
}

impl<T> Omega<T> for MatrixBuf<bool>
where
    T: Clone,
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
}
