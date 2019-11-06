use frank_wolfie_prefer::array::ArrayBuf;

fn main() {
    let data = vec![0, 1, 2, 3, 4, 5];
    let mut array = ArrayBuf::from(data);
    println!("{:?}", array.shape());
    println!("{:?}", array);
    array.reshape([2, 3], 0);
    println!("{:?}", array);
    println!("{:?}", array[[1, 2]]);
    println!("{:?}", array[[0, 2]]);
    array.truncate([5]);
    println!("{:?}", array);
    println!("{:?}", array[[0]]);
}
