fn main(){
    command_line::arguments::print_args();

}

/// The most basic form of 
/// * input of a program is ```through the command line```.
pub mod command_line{

    /// *The args standard library function returns an iterator over the command-line arguments. 
    /// 
    /// * Such an iterator has type Args, and it produces String values. 
    /// The first value produced is the program name, with the path used to reach it. 
    /// The others are the program arguments.
    /// 
    /// Any blank is usually removed; to keep blanks, 
    /// you have to enclose arguments in quotation marks, which will be removed.
    pub mod arguments{
        pub fn print_args(){
            let command_line: std::env::Args = std::env::args(); 

            for argument in command_line {
                println!("[{}]", argument);
            }
        }
    }

}