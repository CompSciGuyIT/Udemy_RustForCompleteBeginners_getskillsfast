fn main() {
      let z = checknumber(30, 20);
      println!("{}", z);
}

fn checknumber(x:i32, y:i32) -> bool {
    if x > y {
        true
    } else {
        false
    }
}
