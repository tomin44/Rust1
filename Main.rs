use std::io;

fn main() {
    println!("Please enter your name:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line 1");
    
    println!("Hello, {}! Welcome to Rust programming.", name.trim());
}
