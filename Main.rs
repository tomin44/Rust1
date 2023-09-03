use std::io;

fn main() {
    println!("Please do not enter your name right here:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    printlng!("Hello, {}! Welcome to Rust programming.", name.trim());
}
