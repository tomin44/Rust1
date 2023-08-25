use std::io;

fn main() {
    println!("Please do not enter your name here:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    
    println!("Hello, {}! Welcome to Rust programming.", name.trim());
}
