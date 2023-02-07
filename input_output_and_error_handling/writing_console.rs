fn main(){
    write_to_console::example();
}


/// Directly use library functions instead of "print!"|"println!" macros
/// to print some text to console
/// 
/// * The "stdout" standard library function returns a handle to the standard output stream of the current process.
///  
/// * On that handle, the "write" function can be applied which gets an argument of "&[u8]" type, 
/// which is a reference to a slice of bytes.
/// * Such bytes are printed to the console as an UTF-8 string. 
/// 
/// To convert both a static string and a dynamic string to a reference to a slice of bytes, 
/// you can use the "as_bytes" function. 
/// 
/// "as_bytes" function just returns the address of the first byte of the string, 
/// and the number of bytes used by the string object. 
/// Such values are already contained in the header of the string object, a
/// nd so this function is extremely efficient.
/// 
/// Finally, notice that the "write" function returns a "Result" type value, 
/// that is, it is a fallible function.
pub mod write_to_console{

    pub fn example(){
        use std::io::Write;
        //ILLEGAL: std::io::stdout().write("Hi").unwrap();
        //ILLEGAL: std::io::stdout().write(String::from("Hi")).unwrap(); 
        std::io::stdout().write("Hello ".as_bytes()).unwrap(); 
        std::io::stdout().write(String::from("world \n !!!").as_bytes()).unwrap();
    }

    pub mod convert_value_to_string{

        /// The "to_string" function allocates a String object, 
        /// whose header is in the stack and whose contents are in the heap. 
        /// Therefore, it is not extremely efficient.
        pub fn using_to_string(){
            let int_str: String = 45.to_string();
            let float_str: String = 4.5.to_string();
            let bool_str: String = true.to_string(); 
            print!("{} {} {}", int_str, float_str, bool_str);
        }
    }

}