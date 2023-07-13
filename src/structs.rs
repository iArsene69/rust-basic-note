// Structs are used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct Colour(u8, u8, u8);

struct Human {
    first_name: String,
    last_name: String,
    is_stressed: bool,
}

impl Human {
    // Construct Human
    fn new(first: &str, last: &str, stressed: bool) -> Human {
        Human {
            first_name: first.to_string(),
            last_name: last.to_string(),
            is_stressed: stressed,
        }
    }

    // Get fullname
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set lastname
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.blue = 255;
    println!("{} {} {}", c.red, c.green, c.blue);

    let mut w = Colour(255, 0, 0);
    w.2 = 255;
    println!("{} {} {}", w.0, w.1, w.2);

    let mut h = Human::new("Johnathan", "Doel", true);
    println!("{}", h.full_name());
    h.set_last_name("Dullahan");
    println!("{} {} is stressed? {}", h.first_name, h.last_name, h.is_stressed);
    println!("Human to tuple: {:?}", h.to_tuple())
}
