// Vectors are resizeable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val
    println!("Single val: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;

    // Add on vectors
    numbers.push(6);
    numbers.push(9);

    println!("{:?}", numbers);

    // Pop on vectors
    numbers.pop();
    println!("Popped: {:?}", numbers);

    // Get vector length
    println!("Vector length is: {}", numbers.len());

    // Vectors are heap allocated
    println!("Vectors occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("slice {:?}", slice); 

    // Loop through vector
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate value
    for x in numbers.iter_mut() {
        *x *= 6;
    }
    println!("Mut Vector {:?}", numbers);
}
