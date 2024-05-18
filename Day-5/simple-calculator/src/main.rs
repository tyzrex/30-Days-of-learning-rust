use std::io::{self, Write};

fn main() {
    // Prompt the user for the first number
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");

    // Prompt the user for the second number
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

    // Prompt the user for the operator
    print!("Enter the operator (+, -, *, /): ");
    io::stdout().flush().unwrap();
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator = operator.trim();

    // Perform the calculation and display the result
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Division by zero is not allowed.");
                return;
            }
        }
        _ => {
            println!("Error: Invalid operator. Please use one of +, -, *, /.");
            return;
        }
    };

    println!(
        "The result of {} {} {} is: {}",
        num1, operator, num2, result
    );
}
