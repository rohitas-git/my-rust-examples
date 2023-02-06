fn main(){
    exit_code::show();
}

/// The most basic form of 
/// * output of a program is its ```return code```.
/// 
/// This program will terminate immediately when it invokes the "exit" function, 
/// and it will return to the launching process the number 107.
/// 
/// command "echo $?", you will get 107 printed on the console in Unix, Linux, Mac
/// The corresponding Windows command is "echo %errorlevel%".
pub mod exit_code{
    pub fn show(){
        std::process::exit(107);
    }
}