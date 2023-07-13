// Functions are used to store blocks of code for re-use

pub fn run() {
    xigreet("Wazzup", "Beijing");

    // Bind functions values to variables
    let get_sum = add(6, 9);
    println!("Sum: {}", get_sum);

    // Closure
    let n3 = 420;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum {}", add_nums(6,9));
}


fn xigreet(greet: &str, object: &str){
    println!("{} {}", greet, object);
}

// function with return
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
    // don't use semicolon `;` when to specify what do you want to return
} 