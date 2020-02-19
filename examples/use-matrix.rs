use frank_wolfe_prefer::linalg::MatBuf;

fn main() {
    let data = vec![0, 1, 2, 3, 4, 5];
    let mut mat_buf = MatBuf::from(data);
    println!("{:?}", mat_buf.shape());
    println!("{:?}", mat_buf);
    mat_buf.reshape([2, 3]);
    println!("{:?}", mat_buf);
    println!("{:?}", mat_buf[[1, 2]]);
    println!("{:?}", mat_buf[[0, 2]]);
}
