use basic_rust::library::{
    fibonacci::{fibonacci_iterative, fibonacci_recursive},
    temperature::convert_temperature,
};

fn main() {
    println!("Celcius in Farenheit: {}", convert_temperature('c', 100.0));
    println!("Farenheit in Celcius: {}", convert_temperature('f', 100.0));
    println!("Fibonacci: {}", fibonacci_recursive(10)); // Recursive is very slow
    println!("Fibonacci: {}", fibonacci_iterative(150)); // Iterative is much faster
}
