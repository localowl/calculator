use std::io;

fn main() {
    let input_one = get_input("Enter first number");
    let operation_str = get_operation();
    let input_two = get_input("Enter second number");

    let operation = match operation_str.trim() {
        "+" => Operations::Add,
        "-" => Operations::Subtract,
        "*" => Operations::Multiply,
        "/" => Operations::Divide,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = calculate(operation, input_one, input_two);
    println!("Result: {}", result);
}

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operations: Operations, input_one: f64, input_two: f64) -> f64 {
    match operations {
        Operations::Add => add(input_one, input_two),
        Operations::Subtract => subtract(input_one, input_two),
        Operations::Multiply => multiply(input_one, input_two),
        Operations::Divide => divide(input_one, input_two),
    }
}

fn add(input_one: f64, input_two: f64) -> f64 {
    input_one + input_two
}

fn subtract(input_one: f64, input_two: f64) -> f64 {
    input_one - input_two
}

fn multiply(input_one: f64, input_two: f64) -> f64 {
    input_one * input_two
}

fn divide(input_one: f64, input_two: f64) -> f64 {
    input_one / input_two
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası!");
    input.trim().parse().expect("Geçersiz sayı!")
}

fn get_operation() -> String {
    println!("İşlemi girin (+, -, *, /): ");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Okuma hatası!");
    operation
}
