// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



// In this case, the problem is told to us by the compiler, 
// the option given to the match expression is consumed by 
// the println! arm.
//
// We can solve this two different ways :
//
// First we can give a "reference" of y to the match expression
// by adding the & symbol next to match.
// 
// Another way (and the way suggested by the compiler), is using 
// the ref keyword inside the pattern-matching part 'some( ref p)'
// but why could'nt we use the & symbol this time ? because we
// are matching on the content of the option y, not a reference to
// it, so the matching would not be valid.
//
// The ref keyword allows specify to the pattern binding to only 
// borrow the bound value, allowing us to return it at the end of
// the function.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
