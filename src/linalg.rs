pub mod mat;
pub mod norm;
mod add_sub;
mod mul;

pub use mat::{Mat, MatBuf};
pub use norm::{Norm, EuclideanNorm};
