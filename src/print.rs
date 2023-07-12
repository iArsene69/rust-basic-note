pub fn run() {
    // Print to console
    println!("Hello i guess?");

    //Basic Formatting
    println!("{} love {}", "I", "Bocchi the Rock!");

    //Positional argument
    println!(
        "{0} are the {1} in the {2} and {3} hate {0}",
        "Lowres member", "worst people", "entire world", "i"
    );

    //Named argument
    println!(
        "{name} like {crush}",
        name = "I",
        crush = "Idk man no one i guess"
    );

    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 69, 69, 69);

    //Placeholder debug traits
    println!("{:?}", (420, "Sixty-nine", true));

    //Basic math
    println!("420 + 69 = {}", 420 + 69);
}
