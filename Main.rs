use std::io;

fn main() {
    println!("Please enter your name here:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read the line");
    printlng!("Hello, {}! Welcome to Rust programming.", name.trim());
}
