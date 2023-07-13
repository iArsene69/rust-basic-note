// Conditionals are used to check the condition of something and act on the result

pub fn run() {
    let age = 18;
    let is_stressed: bool = false;
    let bro_stressed = false;

    // If-Else
    if age >= 21 && is_stressed || bro_stressed {
        println!("Are you ok?");
    } else if age < 21 && is_stressed {
        println!("Hang in there");
    } else {
        println!("Enjoy your live!");
    }

    // Short hand if
    let is_stressed_out = if age >= 21 { true } else { false };

    println!("Bro stressed out? {}", is_stressed_out);
}
