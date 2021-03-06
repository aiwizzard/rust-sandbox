// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign a value
    numbers[2] = 25;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Remove last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("First value: {}", numbers[0]);

    // Get Vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!(
        "Vector occupies {} bytes in memory",
        mem::size_of_val(&numbers)
    );

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Mutated Vector: {:?}", numbers);
}
