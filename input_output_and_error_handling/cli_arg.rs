fn main(){
    command_line::arguments::print_args();

}

/// The most basic form of input of a program is through the command line.
pub mod command_line{

    pub mod arguments{
        pub fn print_args(){
            let command_line: std::env::Args = std::env::args(); 

            for argument in command_line {
                println!("[{}]", argument);
            }
        }
    }

}