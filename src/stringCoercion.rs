fn main() {
    let string1 = String::from("GetSkillsFast");

    coercion(&string1);
}

fn coercion(x: &str) {
    println!("The value is {:?}", x);
}
