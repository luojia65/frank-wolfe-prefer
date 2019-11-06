use core::fmt;
use core::ops::{Index, IndexMut};

// pub struct Array<T, const N: usize, const S: [usize; N]> {
//     data: [T],
// }

#[derive(Debug, Clone)]
pub struct ArrayBuf<T> {
    shape: Shape, // dimension = size.len()
    data: Vec<T>,
}

impl<T> ArrayBuf<T> {
    pub fn new() -> ArrayBuf<T> {
        ArrayBuf {
            shape: Shape::new(),
            data: Vec::new(),
        }
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn len(&self) -> usize {
        self.shape.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shape.len() == 0
    }

    pub fn ndim(&self) -> usize {
        self.shape.ndim()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    // pub fn as_array<const N: usize>(&self) -> Array<T, N> {
    //     unimplemented!()
    // }
}

impl<T> ArrayBuf<T> {
    pub fn truncate<S>(&mut self, new_shape: S)
    where
        S: Into<Shape>,
    {
        let new_shape = new_shape.into();
        let new_len = new_shape.len();
        if new_len <= self.shape.len() {
            self.data.truncate(new_len)
        } else {
            panic!("truncate to a larger shape is not allowed")
        }
        self.shape = new_shape;
    }

    pub fn reshape_with<S, F>(&mut self, new_shape: S, f: F)
    where
        S: Into<Shape>,
        F: FnMut() -> T,
    {
        let new_shape = new_shape.into();
        let new_len = new_shape.len();
        if new_len <= self.shape.len() {
            self.data.truncate(new_len)
        } else {
            self.data.resize_with(new_len, f)
        }
        self.shape = new_shape;
    }
}

impl<T> ArrayBuf<T>
where
    T: Clone,
{
    pub fn reshape<S>(&mut self, new_shape: S, value: T)
    where
        S: Into<Shape>,
    {
        let new_shape = new_shape.into();
        let new_len = new_shape.len();
        if new_len <= self.shape.len() {
            self.data.truncate(new_len)
        } else {
            self.data.resize(new_len, value)
        }
        self.shape = new_shape;
    }
}

impl<T> From<Vec<T>> for ArrayBuf<T> {
    fn from(src: Vec<T>) -> ArrayBuf<T> {
        ArrayBuf {
            shape: Shape::from([src.len()]),
            data: src,
        }
    }
}

impl<T, const N: usize> Index<[usize; N]> for ArrayBuf<T> {
    type Output = T;

    fn index(&self, index: [usize; N]) -> &T {
        &self.data[self.shape.index(index)]
    }
}

impl<T, const N: usize> IndexMut<[usize; N]> for ArrayBuf<T> {
    fn index_mut(&mut self, index: [usize; N]) -> &mut T {
        &mut self.data[self.shape.index(index)]
    }
}

#[derive(Clone)]
pub struct Shape {
    size: Vec<usize>,
}

impl Shape {
    pub fn new() -> Shape {
        Shape { size: Vec::new() }
    }

    pub fn into_inner(self) -> Vec<usize> {
        self.size
    }

    pub fn len(&self) -> usize {
        self.size.iter().product()
    }

    pub fn len_of(&self, dimension: usize) -> usize {
        self.size[dimension]
    }

    pub fn ndim(&self) -> usize {
        self.size.len()
    }

    pub fn index<const N: usize>(&self, index_by_dim: [usize; N]) -> usize {
        check_limits(&self.size, &index_by_dim);
        let mut ans = 0;
        for i in 0..N - 1 {
            ans += index_by_dim[i] * self.size[i + 1]
        }
        ans += index_by_dim[N - 1];
        ans
    }
}

impl<'a> From<&'a [usize]> for Shape {
    fn from(src: &'a [usize]) -> Shape {
        let vec = Vec::from(src);
        Shape { size: vec }
    }
}

impl<const N: usize> From<[usize; N]> for Shape {
    fn from(src: [usize; N]) -> Shape {
        let vec = Vec::from(&src as &[usize]);
        Shape { size: vec }
    }
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.size.iter()).finish()
    }
}

#[inline]
fn check_limits(limit: &[usize], index: &[usize]) {
    if index.len() != limit.len() {
        panic!(
            "index dimension {} does not match shape dimension {}",
            index.len(),
            limit.len()
        )
    }
    for i in 0..limit.len() {
        if index[i] >= limit[i] {
            panic!(
                "index out of bounds at dimension {}: len is {} but index is {}",
                i, limit[i], index[i]
            )
        }
    }
}
