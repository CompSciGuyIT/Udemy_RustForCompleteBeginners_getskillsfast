fn main() {
    let mut vect: Vec<i32> = Vec::new();
    vect.push(5);
    vect.push(8);
    vect.push(3);

    println!("{:?}", vect);

    for i in &vect {
        println!("{:?}", i);
    }

    println!("{:?}", vect[1]);
}
