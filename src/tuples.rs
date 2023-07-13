// Tuples group together values of different types up to 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Insert", "name", 20);

    println!("{} is {} is {}", person.0, person.1, person.2);
}
