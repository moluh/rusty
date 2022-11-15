mod compound_types;
mod shadowing;
// Import only the function that I want
// use shadowing::shadowing;

fn main() {
    println!("Hello, world!");
    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    compound_types::compound();
    shadowing::shadowing();
}
