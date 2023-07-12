// Variables hold primitive data or references to data
// Variables are immutable by default which means you can't reassign it
// Rust is block-scoped language

pub fn run() {
    let name = "[Insert name]";
    let mut age = 19;
    println!("My name is {} and i am {}", name, age);
    age = 20;
    println!("My name is {} and i am {}", name, age);

    // Constant usually use uppercase and need to assign types
    const ID: i32 = 69420;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (user_name, user_age) = ("Insert name", 20);
    println!("{} is {}", user_name, user_age);
}
