fn main() {
    reading_from_console::read_line2(); 
}

/// For command-line oriented programs,
/// a typical way to get input is to read a line from the keyboard until the user presses Enter.
/// Such input may be redirected to read from a file or from the output of another process.

pub mod reading_from_console {

    /// Reading User Input
    pub fn read_line() {
        let mut line = String::new();
        println!("{:?}", std::io::stdin().read_line(&mut line)).expect("Failed to read line");
        println!("[{}]", line.trim());
    }

    /// 1. First, notice that the string printed in the last line spans
    ///     three lines as it contains two end-of-line characters.
    ///
    /// 2. Notice that the "read_line" function appends the typed line to
    ///     the object specified by its argument, instead of overwriting it.
    ///
    /// 3. notice that when the "read_line" function reads the input buffer,
    ///     it clears it, as the original buffer contents are not read again
    ///     when the function is invoked for the second time.
    ///
    /// 4. notice that after every invocation of "read_line", there is an invocation of "unwrap",
    ///     but its return value is ignored.
    ///     Such invocation could be omitted.
    pub fn read_line2() {
        let mut text = format!("First: ");
        let inp = std::io::stdin();
        inp.read_line(&mut text).unwrap();

        text.push_str("Second: "); // shifted to next line due to \n char inclusion at end of input reading
        inp.read_line(&mut text).unwrap();

        println!("{}: {} bytes", text, text.len());
    }

    /*
    However, when this program is compiled, the compiler emits, for both invocations of "read_line", the warning "unused `std::result::Result` which must be used".
    It means that "read_line" returns a value of type "Result" and that value is ignored or not used.

    Rust considers it dangerous to ignore a return value of type "Result", because such a type could represent a runtime error, and so the program logic does not take
    into account such a kind of error. This is dangerous in production code, but it is not appropriate in debug code either, as it hides the errors that you are looking for.

    Therefore, in debug code, it is appropriate to write always at least an “.unwrap()” clause.
    But in production code, matters are not so simple.
    */
    pub fn read_line3() {
        let mut text = format!("First: ");
        let inp = std::io::stdin();
        inp.read_line(&mut text);

        text.push_str("Second: ");
        inp.read_line(&mut text);

        println!("{}: {} bytes", text, text.len());
    }
}
