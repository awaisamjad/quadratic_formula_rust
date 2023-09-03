use std::io;
use num_complex::Complex;

fn parse_float_input(prompt: &str) -> f32 {
    loop {
        // Prompt the user for input
        println!("{}", prompt);

        // Create a new mutable String to store the user input
        let mut input = String::new();

        // Read the input from the user and store it in the `input` variable
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim the input and parse it into a float value
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid float value."),
        };
    }
}

fn main() {
    let a: f32 = parse_float_input("Enter value for 'a':");
    let b: f32 = parse_float_input("Enter value for 'b':");
    let c: f32 = parse_float_input("Enter value for 'c':");
    
    let a_new = Complex::new(a, 0.0);
    let b_new = Complex::new(b, 0.0);
    let c_new = Complex::new(c, 0.0);
    let discriminant:Complex<f32> = ((b_new*b_new) - (4.0 * a_new * c_new)).sqrt();
    let root1:Complex<f32> = (-b + discriminant) / 2.0*a;
    let root2:Complex<f32> = (-b - discriminant) / 2.0*a;
    // Use the parsed float values for further operations
    println!("Root 1: {}  Root 2:{}", root1, root2);
}


