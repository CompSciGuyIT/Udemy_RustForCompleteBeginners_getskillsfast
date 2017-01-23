fn main() {
    let mut vect: Vec<i32> = Vec::with_capacity(3);

    println!("Capacity: {:?}", vect.capacity());

    vect.push(5);
    vect.push(8);
    vect.push(3);

    println!("Capacity: {:?}", vect.capacity());
    println!("Values:  {:?}", vect);
    vect.push(4);

    println!("Capacity: {:?}", vect.capacity());
    println!("Values:  {:?}", vect);
}
