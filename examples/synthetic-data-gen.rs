use frank_wolfe_prefer::{synthetic, linalg::MatrixBuf};

const M: usize = 500;
const N: usize = 40;

fn main() {
    let y_star: MatrixBuf<f32> = synthetic::data(M, N);
    println!("{:?}", (y_star.nrows(), y_star.ncols()));
    println!("{:?} {:?}", y_star[[0, 0]], y_star[[M - 1, N - 1]]);
    let rate = 0.3;
    let omega = synthetic::omega(M, N, rate);
    println!("{:?}", (omega.nrows(), omega.ncols()));
    println!("{:?} {:?}", omega[[0, 0]], omega[[M - 1, N - 1]]);
}
