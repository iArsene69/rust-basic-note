/*  Primitive types on Rust:
integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
character: char
fLoats: f32, f64
boolean: bool
Tuples
Array */

// Rust is statically typed language which means it's nice and not detached from reality like dynamic language such as javascript but also it means it must know every types of variables before compile time, however the compiler usually will infer types automatically based on the value. So kinda like typescript which is decent version of javascript

pub fn run() {
    // Default i32
    let i = 69;

    // Default f64
    let f = 6.9;

    // Add explicit type
    let e: i64 = 420694206942069;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_false = true;

    // Get boolean from expression
    let is_greater: bool = 69 > 420;

    // char
    let a = 'a';
    let smile = '\u{1F600}';

    println!("{:?}", (i, f, e, is_false, is_greater, a, smile));
}
