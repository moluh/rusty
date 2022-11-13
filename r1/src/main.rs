mod shadowing;
// Import only the function that I want
// use shadowing::shadowing;

fn main() {
    println!("Hello, world!");

    shadowing::shadowing()
}
