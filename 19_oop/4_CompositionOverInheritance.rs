mod  composition_over_inheritance{
    // In fact, there are three kinds of inheritance: 
    // *data inheritance, 
    // *method implementation inheritance, 
    // *method interface inheritance.

    // After years of experience, it was found that
    // * Data inheritance created more problems than it solved & so Rust doesn't support it

    // ** Rust uses composition in place of data inheritance
}

pub mod composition{
    // Boxed Text is composed using 
    // Text's field 
    // and method Text::from & Text::draw

    pub struct Text { characters: String }
    impl Text {
        pub fn from(text: &str) -> Text {
            Text { characters: text.to_string() }
        }
        pub fn draw(&self) {
            print!("{}", self.characters);
        }
    }

    pub struct BoxedText {
        text: Text, // *Reuse
        first: char,
        last: char,
    }
    impl BoxedText {
        pub fn with_text_and_borders(
            text: &str, first: char, last: char)
            -> BoxedText
        {
            BoxedText {
                text: Text::from(text), // *Reuse 
                first: first,
                last: last,
            }
        }

        pub fn draw(&self) {
            print!("{}", self.first);
            self.text.draw(); // *Reuse
            print!("{}", self.last);
        }
    }
}

use composition::*;

fn main(){
    let greeting = Text::from("Hello");
    greeting.draw();

    let boxed_greeting =
        BoxedText::with_text_and_borders("Hi", '[', ']');
    print!(", ");
    boxed_greeting.draw();
}