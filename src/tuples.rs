fn main() {
    let tuple = ('h', 38, "word");
    println!("{:?}", tuple.2);

    let (a, b, c) = tuple;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
