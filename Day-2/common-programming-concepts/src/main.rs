fn main() {
    // Variable declaration and initialization
    let number: i32 = 10;
    let float_number: f64 = 3.14;
    let character: char = 'A';
    let is_rust_fun: bool = true;

    // Printing variables
    println!("Integer Number: {}", number);
    println!("Float Number: {}", float_number);
    println!("Character: {}", character);
    println!("Is Rust Fun?: {}", is_rust_fun);

    // Basic arithmetic
    let sum = number + 5;
    let product = number * 2;
    println!("Sum: {}", sum);
    println!("Product: {}", product);

    // Conditional statements
    if is_rust_fun {
        println!("I'm having fun with Rust!");
    } else {
        println!("Rust is not fun for me.");
    }

    // Looping
    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }

    // Arrays
    let array: [i32; 4] = [1, 2, 3, 4];
    println!("Array: {:?}", array);

    // Vector
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("Vector: {:?}", vector);

    // Functions
    let result = add_numbers(3, 5);
    println!("Result of adding numbers: {}", result);
}

// Function to add two numbers
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
