use frank_wolfe_prefer::{
    linalg::{MatrixBuf, EuclideanNorm, Norm}, 
    synthetic, 
    Omega
};

const M: usize = 500;
const N: usize = 40;

fn main() {
    let y_star_0: MatrixBuf<f32> = synthetic::data(M, N);
    println!("{:?}", (y_star_0.nrows(), y_star_0.ncols()));
    println!("{:?} {:?}", y_star_0[[0, 0]], y_star_0[[M - 1, N - 1]]);
    let rate = 0.3;
    let omega = synthetic::omega(M, N, rate);
    println!("{:?}", (omega.nrows(), omega.ncols()));
    println!("{:?} {:?}", omega[[0, 0]], omega[[M - 1, N - 1]]);
    let y_star = omega.p_omega(y_star_0, 0.0);
    println!("{:?}", (y_star.nrows(), y_star.ncols()));
    println!("{:?} {:?}", y_star[[0, 0]], y_star[[M - 1, N - 1]]);
    println!("{:?}", &y_star[0]);
    let norm = EuclideanNorm::norm(&y_star[0]);
    println!("Norm of Y[0, ..]: {:?}", norm);
}
