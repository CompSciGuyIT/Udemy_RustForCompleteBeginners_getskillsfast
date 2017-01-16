fn main() {
    let mut num = 3;
    println!("{}", num);
    my_func();
    num = num_func();
    println!("{}", num);
}

fn my_func() {
    let num = 5;
    println!("The number is {}", num);
}

fn num_func() -> i32 {
    let number = 7;
    number
}
