// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

fn call_me(num: i32) {// rust requires that all parts of a function's signature have type annotations
    // is missing the type annotation of num
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
