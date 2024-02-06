// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.



// In this case the Book struct holds to string slice, which are both references. The Compiler
// must be sure that the 2 reference contained in this struct are not poiting to undefined or 
// undesirable data (what rust calls a dangling reference). To accomplish this we add the generic
// lifetime a' in our struct declaration and apply it to both reference. what we are doing here
// is telling the compiler "A Book struct cannot have a longer lifetime than any of the 2 reference
// it contains

struct Book <'a> {
    author: &'a str,
    title: &'a str,
}



fn main() {
    let name = String::from("Jill Smith");
    
    
    let title = String::from("Fish Flying");
    
    

    let book = Book { author: &name, title: &title };

    

    println!("{} by {}", book.title, book.author);
}
