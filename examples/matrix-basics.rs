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
    let data = vec![0, 1, 2, 3, 4, 5];
    let mut array = ArrayBuf::from(data);
    array.truncate([3, 2]);
    let m3 = MatrixBuf::from(array);
    let data = vec![9, 8, 7, 7, 7, 7];
    let mut array = ArrayBuf::from(data);
    array.truncate([3, 2]);
    let m4 = MatrixBuf::from(array);
    println!("{:?}", m3 + m4);
}
