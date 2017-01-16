fn main() {
    let num:i8 = 20;
    if num < 1 {
        println!("The number is less than one.");
    } else if num > 50 && num < 70{
        println!("The number is greater than fifty and less than seventy.");
    } else if num >= 70{
        println!("The number is seventy or greater.");
    } else {
        println!("The number is greater than zero and less than fifty one.");
    }
}
