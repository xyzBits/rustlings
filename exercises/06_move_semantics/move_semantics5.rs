// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;// first mutable borrow occurs here
    *y += 100;// first mutable borrow occurs here

    let z = &mut x;// second mutable borrow occurs

    *z += 1000;// second mutable occurs here
    assert_eq!(x, 1200);
}
