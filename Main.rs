use std::io;

fn main() {
    println!("Please enter your name:");
    
    let mute name = String::new();
    io::stdin().read_line(&mute name).expect("Failed to read line");
    
    println!("Hello, {}! Welcome to Rust programming.", name.trim());
}