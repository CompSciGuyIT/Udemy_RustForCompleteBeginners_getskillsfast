enum Person {
    Doctor,
    Height(i32)
}

fn main() {
    let john = Person::Doctor;
    let ciba = Person::Height(22);

    checktheparam(john);
    checktheparam(ciba);
}

fn checktheparam(p: Person) {
    match p {
        Person::Doctor => println!("They are a Doctor."),
        Person::Height(i) => println!("The person has a height of {:?}", i)

    }
}
