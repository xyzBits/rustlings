// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut x = 3;// first assignment to 'x'
    println!("Number {}", x);// cannot assign twice to immutable variable 'x'
    x = 5; // don't change this line
    println!("Number {}", x);
}
