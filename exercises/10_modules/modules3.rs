// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.



// Here we can use 2 syntax to correctly import the std::time Systemtime
// And UNIX EPOCH, we can use the glob operator * to import all from the
// std::time crate, or we can specify the necessary part of the crate we
// want using the {} symbols


// TODO: Complete this use statement
use std::time::{UNIX_EPOCH,SystemTime};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
