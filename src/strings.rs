// primitive str is immutable fixed length string somewhere in the memory
// String is growable, heap-allocated data structure uses when you want to modify or own

pub fn run(){
    // Primitive str
    let str = "Wassup dawg";
    println!("{}", str);

    // String
    let mut text = String::from("I'm ");

    // Get length
    println!("Text length: {}", text.len());

    // push str
    text.push_str("fine ");

    // push char
    text.push('\u{1F600}');

    println!("{}", text);

    // String method (for more read docs)

    // capacity
    println!("Capacity: {}", text.capacity());

    // is_empty
    println!("Is empty: {}", text.is_empty());

    // contains
    println!("Contains 'fine': {}", text.contains("fine"));

    // replace
    println!("{}", text.replace("fine", "not fine"));

    // loop through string by whitespace
    for word in text.split_whitespace() {
        println!("{}", word);
    }

    // create String with capacity
    let mut text_cap = String::with_capacity(10);
    text_cap.push('\u{1F600}');
    text_cap.push_str(" Smile");

    println!("{}", text_cap);

    // assertion testing
    assert_eq!(10, text_cap.len());
    assert_ne!(6, text_cap.capacity());
}