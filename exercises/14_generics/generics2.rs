// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



// To make the Wrapper Type accept different data type, we need to modify
// its struct declaration and the new() method function signature.
// First we need to declare the generic data type by adding <T> to both
// So that the Compiler knows of their existence.
//
// We have to also declare this in the implementation of the function 
// new, we could name it the same as in the struct declaration or another
// name, but we name it T also by convention.


struct Wrapper<T> {
    value: T,
}

impl<U> Wrapper<U> {
    pub fn new(value: U) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
