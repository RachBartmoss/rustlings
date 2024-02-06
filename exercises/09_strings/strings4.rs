// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



// In this case, each case is special , so i will comment each case separately :


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // Here we have a string litteral, meaning it is stored as a static strings
                          // slice

    string("red".to_string()); // Here we have a literal going trough the to_string method that
                               // that returns a String from another variable, by default by creating
                               // a new string in the heap and formatting the transformed variable
                               // into it

    string(String::from("hi")); // Here we use the String::from associated function taking a &str as an argument
                                // this function by default uses the to_owned method on the &str parameter to copy
                                // the parameter content on the heap for a String

    string("rust is fun!".to_owned()); // Same as before, transforms the borrowed &str into an owned String
                                       // Ususally by cloning


    string_slice("nice weather".into()); // Here the choice of String or String_Slice is irrelevant, as the
                                         // Into method convert the data by inferring the correct type.
                                         // In this case using the function parameters type to infer into what
                                         // type it must convert the &str


    string(format!("Interpolation {}", "Station")); // Here the use of the format! macro concatenate the content
                                                    // of both string literals into a single true String


    string_slice(&String::from("abc")[0..1]); // Here we take a slice of a String reference, created from a litteral
                                              // the final data is a reference to a portion of the string, so a &str


    string_slice("  hello there ".trim()); // The trim() method applied to a &str here returns the same type of data
                                           // a &str in this case



    string("Happy Monday!".to_string().replace("Mon", "Tues")); // Here we start with a literal, convert it with the to
                                                                // String method, then send the result to the replace method
                                                                // that too returns a string




    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // This time once again, the method to_lowercase applied to &str
                                                     // returns a String
}
