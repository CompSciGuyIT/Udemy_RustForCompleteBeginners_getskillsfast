fn main() {
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", array);
    let new_array1 = &array[..];
    let new_array2 = &array[5..];
    let new_array3 = &array[..5];
    let new_array4 = &new_array3[3..];
    println!("{:?}", new_array1);
    println!("{:?}", new_array2);
    println!("{:?}", new_array3);
    println!("{:?}", new_array4);
}
