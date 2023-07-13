// Loops are used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    //infinite loop
    loop {
        count += 1;
        println!("Number {}", count);
        // This will loop forever until a condition is provided and do break, so don't run it without condition

        if count == 5 {
            break;
        }
    }

    //While loop {FizzBuzz}
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{}", count);
        }

        // inc

        count += 1;
    }

    // For range loop
    for i in 0..100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{}", i);
        }
    }
}