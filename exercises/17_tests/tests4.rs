// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.



// here the first test function simply verify that the rectangle Constructor, given a width : x
// and a heigth : y , truly returns a  rectangle struct with width x and heigth y, so we need
// to check for both fields of the struct during our test.
//
// For the second and third test, we must first notice that the constructor function uses the
// panic!() macro if width OR heigth is negative (inferior to 0), this macro terminate the 
// program and to make a test that checks if the function makes the programm panic, we must
// use the [should_panic] attribute. We can even be more specific and feed the attribute a String
// with the expected keyword so that should_panic checks for this String in the Panic!() message and
// only validate if both message are the same, this could be useful if we want to confirm that the panic
// we catch is truly the panic we intended to catch.

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width , 10); // check width
        assert_eq!(rect.height , 20); // check height
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}
