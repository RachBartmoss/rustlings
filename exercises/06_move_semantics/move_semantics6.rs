// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



// The String data, when used in the get_char function is taken and moved without
// reference. when the get_char() function is finished, the variable date becomes unusable
// So we make the get_char() function argument a reference of a string, so that the data
// variable is still available for the string_uppercase() function
//
// Another error then pops up in the string_uppercase function telling us that 
// the &data.to_uppercase() created a value that was dropped at the end of the statement.
// the &data.to_uppercase() actually borrows the result of the to_uppercase() method
// by creating a temporary, and all temporary are dropped at the end of the statement.
// To correct this, we need to give string_uppercase ownership of the data variable, and
// since we can only move reference around in this exercise, i just remove the reference
// symbol from the function and its call

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
