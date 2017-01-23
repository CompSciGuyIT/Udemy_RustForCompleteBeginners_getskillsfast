use std::io;

fn main() {
    println!("Please enter a number");

    let mut num = String::new();

    io::stdin().read_line(&mut num);

    // This loop removes the \r and \n added to the string
    for i in 0..2 {
        num.pop();
    }
    print!("The number you entered is {:?}", num);
}
