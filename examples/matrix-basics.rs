use frank_wolfe_prefer::{array::ArrayBuf, linalg::MatrixBuf};

fn main() {
    let data = vec![0, 1, 2, 3, 4, 5];
    let mut array = ArrayBuf::from(data);
    array.reshape([3, 2], 0);
    println!("{:?}", array);
    let m1 = MatrixBuf::from(array);
    println!("{:?}", m1);
    let mut m2 = m1.clone();
    m2.transpose();
    println!("{:?}", m2);
    println!("{:?}", m1 * m2);
}
