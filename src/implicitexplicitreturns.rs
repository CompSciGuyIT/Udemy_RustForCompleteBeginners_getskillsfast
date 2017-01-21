fn main() {
    let num1 = 7;
    let num2 = 9;
    println!("{:?}", addition(num1, num2));
    println!("{:?}", greatest(num1, num2));
}

fn addition(x:i32, y:i32) -> i32 {
    x + y               // implicit return
}

fn greatest(x:i32, y:i32) -> i32 {
    if x > y {
        return x;       // explicit return
    }
    y
}
