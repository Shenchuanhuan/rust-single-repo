fn main() {
    println!("Hello, world!");

    another_function();
    another_function_parameters(5);
    print_labeled_measurement(10, 'h');

    let x = five();
    println!("The value of x is {}", x);

    let y = plus_one(10);
    println!("The value of y is {}", y);
}

// Function definitions in Rust start with fn
// Rust doesn’t care where you define your functions, only that they’re defined somewhere
fn another_function() {
    println!("another function.");
}

// In function signatures, you must declare the type of each parameter
fn another_function_parameters(x: i32) {
    println!("The vaule of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Expressions do not include ending semicolons.If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. 

fn five() -> i32 {
    10
}

fn plus_one(x: i32) -> i32 {
    x + 1
}