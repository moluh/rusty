// References and mutability
// Talk about:

// Can't change a value unless it's mutable*
// Passing a reference to a value is done with &
// Passing a mutable ref is done with &mut
// * will dereference
// When to pass a reference and when to move
fn main() {
    let mut byte = 0u8;
    println!("byte {}!", byte);

    change(&mut byte);

    fn change(b: &mut u8) {
        *b = 1; // deref `b` and give it a new value
        println!("*b", *b);
    }
}
