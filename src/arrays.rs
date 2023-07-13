// Arrays are fixed list where elements are the same data types

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val
    println!("Single val: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get array length
    println!("Array length is: {}", numbers.len());

    // Arrays size on memory
    println!("Arrays occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("slice {:?}", slice); 
}
