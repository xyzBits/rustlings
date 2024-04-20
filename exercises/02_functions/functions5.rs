// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {// expected i32, found ()
    // implicitly returns '()' as its body has no tail or return expression
    return num * num;
}
