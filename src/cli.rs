use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let object = "Beijing";

    if command == "wazzup" {
        println!("{} {}", command, object);
    }


}