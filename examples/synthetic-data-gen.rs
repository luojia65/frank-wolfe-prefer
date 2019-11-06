use frank_wolfe_prefer::{synthetic, linalg::MatrixBuf};

const M: usize = 500;
const N: usize = 40;

fn main() {
    let y_star: MatrixBuf<f32> = synthetic::generate(M, N);
    println!("{:?}", (y_star.nrows(), y_star.ncols()));
    println!("{:?} {:?}", y_star[[0, 0]], y_star[[M - 1, N - 1]]);
}
