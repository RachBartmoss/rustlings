// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.



// In this exercise we see that we need to implement 4 enumerations variants to 
// comply with the rest of the code, these variants will then be used by the 
// process method to operate on the state struct and modify the corresponding
// struct variable, so we need to declare the variants with the same type as 
// the struct member , a tri-u8 tuple for color-changing, a Point struct for moving 
// and a String for echoing into the state's message.
//
// we then create a match case for all these variant, we also bind the value contained
// in the variant to extract data so we can add them to the struct variable (extracting
// the color using x,y,z to then move them in the struct's color member)

enum Message {
    ChangeColor(u8,u8,u8),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(x,y,z) => {
                self.color.0 = x;
                self.color.1 = y;
                self.color.2 = z;
            }
            Message::Echo(string) => self.message = string,
            Message::Move(point) => self.position = point,
            Message::Quit => self.quit = true,

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
