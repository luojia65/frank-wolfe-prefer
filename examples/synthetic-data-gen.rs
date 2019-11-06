use frank_wolfe_prefer::{array::ArrayBuf, linalg::MatrixBuf};

const M: usize = 500;
const N: usize = 40;

fn main() {
    let mut uv = vec![0f32; M];
    for i in 0..M {
        uv[i] = rand::random();
    }
    let mut ua = ArrayBuf::from(uv);
    ua.reshape([M, 1], 0.0);
    let u = MatrixBuf::from(ua);
    let mut vv = vec![0f32; N];
    for i in 0..N {
        vv[i] = rand::random();
    }
    let mut va = ArrayBuf::from(vv);
    va.reshape([1, N], 0.0);
    let v = MatrixBuf::from(va);
    let y_star = u * v;
    println!("{:?}", (y_star.nrows(), y_star.ncols()));
    println!("{:?} {:?}", y_star[[0, 0]], y_star[[M - 1, N - 1]]);
}
