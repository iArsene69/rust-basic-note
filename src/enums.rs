// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn player(m: Movement){
    // Perform action depending on info
    match m {
        Movement::Up => println!("Up we go!"),
        Movement::Down => println!("Down we go!"),
        Movement::Left => println!("Left we go!"),
        Movement::Right => println!("Right we go!")
    }
}

pub fn run() {
    let player1 = Movement::Up;
    let player2 = Movement::Right;
    let player3 = Movement::Left;
    let player4 = Movement::Down;

    player(player1);
    player(player2);
    player(player3);
    player(player4);
}