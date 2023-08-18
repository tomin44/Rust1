use std::io;

fn main() {
    println!("Please enter here your name:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read next line");
    
    println!("Hello, {}! Welcome to Rust programming.", name.trim());
}
