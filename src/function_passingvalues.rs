fn main() {
    let num = multiply_func(4, 8);
    println!("{}", num);
}

fn multiply_func(x:i32, y:i32) -> i32 {
    let number = x * y;
    number
}
