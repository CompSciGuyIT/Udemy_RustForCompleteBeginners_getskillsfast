fn main() {
    let mut array = [12, 32, 54, 24, 11, 79];
    println!("{:?}", array);
    array[3] = 90;
    println!("{:?}", array);
    let array2 = [3, 3, 3, 3];
    let array3 = [3; 4];
    println!("{:?}", array2);
    println!("{:?}", array3);
}
