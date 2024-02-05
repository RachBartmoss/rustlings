// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// Rust returns implicitly the last expression of a function, here by removing the ; we made
// the 'num * num' the last expression. if you leave the ';' the 'num * num' woudld be a 
// statement and therefore return nothing but ()

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
